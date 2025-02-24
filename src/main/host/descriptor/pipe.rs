use atomic_refcell::AtomicRefCell;
use nix::errno::Errno;
use std::sync::Arc;

use crate::cshadow as c;
use crate::host::descriptor::shared_buf::SharedBuf;
use crate::host::descriptor::{
    FileMode, FileState, FileStatus, StateEventSource, StateListenerFilter,
};
use crate::host::memory_manager::MemoryManager;
use crate::host::syscall_types::{PluginPtr, SyscallError, SyscallResult};
use crate::utility::event_queue::{EventQueue, Handle};
use crate::utility::stream_len::StreamLen;

pub struct PipeFile {
    buffer: Option<Arc<AtomicRefCell<SharedBuf>>>,
    event_source: StateEventSource,
    state: FileState,
    mode: FileMode,
    status: FileStatus,
    write_mode: WriteMode,
    buffer_event_handle: Option<Handle<(FileState, FileState)>>,
    // should only be used by `OpenFile` to make sure there is only ever one `OpenFile` instance for
    // this file
    has_open_file: bool,
}

impl PipeFile {
    /// Create a new [`PipeFile`]. The new pipe must be initialized using
    /// [`PipeFile::connect_to_buffer`] before any of its methods are called.
    pub fn new(mode: FileMode, status: FileStatus) -> Self {
        Self {
            buffer: None,
            event_source: StateEventSource::new(),
            state: FileState::ACTIVE,
            mode,
            status,
            write_mode: WriteMode::Stream,
            buffer_event_handle: None,
            has_open_file: false,
        }
    }

    pub fn get_status(&self) -> FileStatus {
        self.status
    }

    pub fn set_status(&mut self, status: FileStatus) {
        self.status = status;
    }

    pub fn mode(&self) -> FileMode {
        self.mode
    }

    pub fn has_open_file(&self) -> bool {
        self.has_open_file
    }

    pub fn supports_sa_restart(&self) -> bool {
        true
    }

    pub fn set_has_open_file(&mut self, val: bool) {
        self.has_open_file = val;
    }

    pub fn max_size(&self) -> usize {
        self.buffer.as_ref().unwrap().borrow().max_len()
    }

    pub fn close(&mut self, event_queue: &mut EventQueue) -> Result<(), SyscallError> {
        // drop the event listener handle so that we stop receiving new events
        self.buffer_event_handle.take().unwrap().stop_listening();

        // if open for writing, inform the buffer that there is one fewer writers
        if self.mode.contains(FileMode::WRITE) {
            self.buffer
                .as_ref()
                .unwrap()
                .borrow_mut()
                .remove_writer(event_queue);
        }

        // if open for reading, inform the buffer that there is one fewer readers
        if self.mode.contains(FileMode::READ) {
            self.buffer
                .as_ref()
                .unwrap()
                .borrow_mut()
                .remove_reader(event_queue);
        }

        // no need to hold on to the buffer anymore
        self.buffer = None;

        // set the closed flag and remove the active, readable, and writable flags
        self.copy_state(
            FileState::CLOSED | FileState::ACTIVE | FileState::READABLE | FileState::WRITABLE,
            FileState::CLOSED,
            event_queue,
        );

        Ok(())
    }

    pub fn read<W>(
        &mut self,
        mut bytes: W,
        offset: libc::off_t,
        event_queue: &mut EventQueue,
    ) -> SyscallResult
    where
        W: std::io::Write + std::io::Seek,
    {
        // pipes don't support seeking
        if offset != 0 {
            return Err(nix::errno::Errno::ESPIPE.into());
        }

        // if the file is not open for reading, return EBADF
        if !self.mode.contains(FileMode::READ) {
            return Err(nix::errno::Errno::EBADF.into());
        }

        let num_read = self
            .buffer
            .as_ref()
            .unwrap()
            .borrow_mut()
            .read(&mut bytes, event_queue)?;

        // the read would block if all:
        //  1. we could not read any bytes
        //  2. we were asked to read >0 bytes
        //  3. there are open descriptors that refer to the write end of the pipe
        if usize::from(num_read) == 0
            && bytes.stream_len_bp()? != 0
            && self.buffer.as_ref().unwrap().borrow().num_writers() > 0
        {
            Err(Errno::EWOULDBLOCK.into())
        } else {
            Ok(num_read.into())
        }
    }

    pub fn write<R>(
        &mut self,
        mut bytes: R,
        offset: libc::off_t,
        event_queue: &mut EventQueue,
    ) -> SyscallResult
    where
        R: std::io::Read + std::io::Seek,
    {
        // pipes don't support seeking
        if offset != 0 {
            return Err(nix::errno::Errno::ESPIPE.into());
        }

        // if the file is not open for writing, return EBADF
        if !self.mode.contains(FileMode::WRITE) {
            return Err(nix::errno::Errno::EBADF.into());
        }

        let mut buffer = self.buffer.as_ref().unwrap().borrow_mut();

        if buffer.num_readers() == 0 {
            return Err(nix::errno::Errno::EPIPE.into());
        }

        if self.write_mode == WriteMode::Packet && !self.status.contains(FileStatus::DIRECT) {
            // switch to stream mode immediately, regardless of whether the buffer is empty or not
            self.write_mode = WriteMode::Stream;
        } else if self.write_mode == WriteMode::Stream && self.status.contains(FileStatus::DIRECT) {
            // in linux, it seems that pipes only switch to packet mode when a new page is added to
            // the buffer, so we simulate that behaviour for when the first page is added (when the
            // buffer is empty)
            if !buffer.has_data() {
                self.write_mode = WriteMode::Packet;
            }
        }

        let len = bytes.stream_len_bp()? as usize;

        match self.write_mode {
            WriteMode::Stream => buffer.write_stream(bytes.by_ref(), len, event_queue),
            WriteMode::Packet => {
                let mut num_written = 0;

                loop {
                    // the number of remaining bytes to write
                    let bytes_remaining = len - num_written;

                    // if there are no more bytes to write (pipes don't support 0-length packets)
                    if bytes_remaining == 0 {
                        break Ok(num_written.into());
                    }

                    // split the packet up into PIPE_BUF-sized packets
                    let bytes_to_write = std::cmp::min(bytes_remaining, libc::PIPE_BUF);

                    if let Err(e) = buffer.write_packet(bytes.by_ref(), bytes_to_write, event_queue)
                    {
                        // if we've already written bytes, return those instead of an error
                        if num_written > 0 {
                            break Ok(num_written.into());
                        }
                        break Err(e);
                    }

                    num_written += bytes_to_write;
                }
            }
        }
    }

    pub fn ioctl(
        &mut self,
        request: u64,
        _arg_ptr: PluginPtr,
        _memory_manager: &mut MemoryManager,
    ) -> SyscallResult {
        log::warn!("We do not yet handle ioctl request {} on pipes", request);
        Err(Errno::EINVAL.into())
    }

    pub fn connect_to_buffer(
        arc: &Arc<AtomicRefCell<Self>>,
        buffer: Arc<AtomicRefCell<SharedBuf>>,
        event_queue: &mut EventQueue,
    ) {
        let weak = Arc::downgrade(arc);
        let pipe = &mut *arc.borrow_mut();

        pipe.buffer = Some(buffer);

        if pipe.mode.contains(FileMode::WRITE) {
            pipe.buffer
                .as_ref()
                .unwrap()
                .borrow_mut()
                .add_writer(event_queue);
        }

        if pipe.mode.contains(FileMode::READ) {
            pipe.buffer
                .as_ref()
                .unwrap()
                .borrow_mut()
                .add_reader(event_queue);
        }

        // remove any state flags that aren't relevant to us
        let monitoring = pipe.filter_state(FileState::READABLE | FileState::WRITABLE);

        let handle = pipe.buffer.as_ref().unwrap().borrow_mut().add_listener(
            monitoring,
            StateListenerFilter::Always,
            move |state, _changed, event_queue| {
                // if the file hasn't been dropped
                if let Some(pipe) = weak.upgrade() {
                    let mut pipe = pipe.borrow_mut();

                    // if the pipe is already closed, do nothing
                    if pipe.state.contains(FileState::CLOSED) {
                        return;
                    }

                    pipe.copy_state(monitoring, state, event_queue);
                }
            },
        );

        pipe.buffer_event_handle = Some(handle);

        // update the pipe file's initial state based on the buffer's state
        let buffer_state = pipe.buffer.as_ref().unwrap().borrow_mut().state();
        pipe.state.insert(pipe.filter_state(buffer_state));
    }

    pub fn add_listener(
        &mut self,
        monitoring: FileState,
        filter: StateListenerFilter,
        notify_fn: impl Fn(FileState, FileState, &mut EventQueue) + Send + Sync + 'static,
    ) -> Handle<(FileState, FileState)> {
        self.event_source
            .add_listener(monitoring, filter, notify_fn)
    }

    pub fn add_legacy_listener(&mut self, ptr: *mut c::StatusListener) {
        self.event_source.add_legacy_listener(ptr);
    }

    pub fn remove_legacy_listener(&mut self, ptr: *mut c::StatusListener) {
        self.event_source.remove_legacy_listener(ptr);
    }

    pub fn state(&self) -> FileState {
        self.state
    }

    fn filter_state(&self, mut state: FileState) -> FileState {
        // if not open for reading, remove the readable flag
        if !self.mode.contains(FileMode::READ) {
            state.remove(FileState::READABLE);
        }

        // if not open for writing, remove the writable flag
        if !self.mode.contains(FileMode::WRITE) {
            state.remove(FileState::WRITABLE);
        }

        state
    }

    fn copy_state(&mut self, mask: FileState, state: FileState, event_queue: &mut EventQueue) {
        let old_state = self.state;

        // remove any flags that aren't relevant
        let state = self.filter_state(state);

        // remove the masked flags, then copy the masked flags
        self.state.remove(mask);
        self.state.insert(state & mask);

        self.handle_state_change(old_state, event_queue);
    }

    fn handle_state_change(&mut self, old_state: FileState, event_queue: &mut EventQueue) {
        let states_changed = self.state ^ old_state;

        // if nothing changed
        if states_changed.is_empty() {
            return;
        }

        self.event_source
            .notify_listeners(self.state, states_changed, event_queue);
    }
}

#[derive(Debug, PartialEq, Eq)]
enum WriteMode {
    Stream,
    Packet,
}
