# Reconstruct relevant flags to pass through to LLVM for header parsing.
# Unfortunately CMAKE_C_FLAGS doesn't include everything; we need to add
# more, below.
separate_arguments(LLVM_FLAGS UNIX_COMMAND "${CMAKE_C_FLAGS}")

# Reconstruct "-I" flags.
get_target_property(INCLUDE_DIRECTORIES shadow-c INCLUDE_DIRECTORIES)
# Add -I prefix to each directory
foreach(INCLUDE_DIRECTORY IN LISTS INCLUDE_DIRECTORIES)
    list(APPEND LLVM_FLAGS "-I${INCLUDE_DIRECTORY}")
endforeach(INCLUDE_DIRECTORY)

# Reconstruct "-D" flags.
get_directory_property(COMPILE_DEFINITIONS COMPILE_DEFINITIONS)
# Add -D prefix to each definition
foreach(COMPILE_DEFINITION IN LISTS COMPILE_DEFINITIONS)
    list(APPEND LLVM_FLAGS "-D${COMPILE_DEFINITION}")
endforeach(COMPILE_DEFINITION)

# Generate wrapper.rs in the source tree.
add_custom_command(OUTPUT wrapper.rs
    COMMAND bindgen
        # Haven't decided how to handle glib struct types yet. Avoid using them
        # until we do.
        --blacklist-type "_?GQueue"
        # Needs GQueue
        --blacklist-type "_?Socket.*"
        # Needs Socket
        --blacklist-type "_?CompatSocket.*"

        # Uses atomics, which bindgen doesn't translate correctly.
        # https://github.com/rust-lang/rust-bindgen/issues/2151
        --blacklist-type "atomic_bool"
        --blacklist-type "_?ShimThreadSharedMem"
        --blacklist-type "_?ShimProcessSharedMem"
        --blacklist-function "thread_sharedMem"

        --whitelist-function "affinity_.*"
        --whitelist-function "thread_.*"
        --whitelist-function "descriptor_close"
        --whitelist-function "descriptor_unref"
        --whitelist-function "descriptor_getHandle"
        --whitelist-function "descriptor_setHandle"
        --whitelist-function "descriptor_setOwnerProcess"
        --whitelist-function "descriptor_shutdownHelper"
        --whitelist-function "host_.*"
        # Needs CompatSocket
        --blacklist-function "host_.*Interface"

        # used by shadow's main function
        --whitelist-function "main_.*"
        --whitelist-function "shmemcleanup_tryCleanup"
        --whitelist-function "controller_(new|free|run)"
        --whitelist-function "runConfigHandlers"
        --whitelist-function "rustlogger_new"

        --whitelist-function "workerpool_updateMinRunahead"

        --whitelist-function "process_.*"
        --whitelist-function "shadow_logger_getDefault"
        --whitelist-function "shadow_logger_shouldFilter"
        --whitelist-function "statuslistener_ref"
        --whitelist-function "statuslistener_unref"
        --whitelist-function "statuslistener_onStatusChanged"
        --whitelist-function "syscallcondition_new"
        --whitelist-function "syscallcondition_unref"
        --whitelist-function "syscallcondition_getActiveFile"
        --whitelist-function "syscallcondition_setActiveFile"
        --whitelist-function "syscallhandler_.*"
        --whitelist-function "worker_.*"
        --whitelist-function "workerc_.*"
        --whitelist-function "packet_getSourceIP"
        --whitelist-function "packet_getDestinationIP"
        --whitelist-function "packet_getSourcePort"
        --whitelist-function "packet_getDestinationPort"
        --whitelist-function "packet_getHeaderSize"
        --whitelist-function "packet_getPayloadLength"
        --whitelist-function "packet_getProtocol"
        --whitelist-function "packet_getTCPHeader"
        --whitelist-function "packet_copyPayloadShadow"
        # Needs GQueue
        --blacklist-function "worker_finish"
        --blacklist-function "worker_bootHosts"
        --blacklist-function "worker_freeHosts"

        --blacklist-function "syscallhandler_new"
        --blacklist-function "syscallhandler_ref"
        --blacklist-function "syscallhandler_unref"
        --blacklist-function "syscallhandler_make_syscall"

        --whitelist-function "return_code_for_signal"

        --whitelist-type "Host"
        --whitelist-type "PluginPtr"
        --whitelist-type "Status"
        --whitelist-type "StatusListener"
        --whitelist-type "SysCall.*"
        --whitelist-type "LegacyDescriptor"
        --whitelist-type "Trigger"
        --whitelist-type "TriggerType"
        --whitelist-type "LogInfoFlags"
        --whitelist-type "SimulationTime"
        --whitelist-type "SchedulerPolicyType"
        --whitelist-type "ProtocolTCPFlags"
        --whitelist-var "CONFIG_PIPE_BUFFER_SIZE"
        --whitelist-var "SYSCALL_IO_BUFSIZE"
        --opaque-type "LegacyDescriptor"
        --opaque-type "CompatDescriptor"
        --opaque-type "OpenFile"
        --opaque-type "ConfigOptions"
        --opaque-type "Logger"
        --opaque-type "DescriptorTable"
        --opaque-type "MemoryManager"
        --opaque-type "Random"
        --opaque-type "GList"
        --blacklist-type "Logger"
        --blacklist-type "CompatDescriptor"
        --whitelist-type "WorkerC"
        --opaque-type "WorkerC"
        --whitelist-type "WorkerPool"
        --opaque-type "WorkerPool"
        --blacklist-type "Arc_AtomicRefCell_AbstractUnixNamespace"
        --blacklist-type "HashSet_String"
        --disable-header-comment
        --raw-line "/* automatically generated by rust-bindgen */"
        --raw-line "use crate::host::descriptor::CompatDescriptor;"
        --raw-line "use crate::host::descriptor::OpenFile;"
        --raw-line "use crate::host::descriptor::descriptor_table::DescriptorTable;"
        --raw-line "use crate::host::memory_manager::MemoryManager;"
        --raw-line "use crate::core::support::configuration::ConfigOptions;"
        --raw-line "use crate::utility::random::Random;"
        --raw-line "use log_bindings::Logger;"
        --raw-line "use std::sync::Arc;"
        --raw-line "use std::collections::HashSet;"
        --raw-line "use atomic_refcell::AtomicRefCell;"
        --raw-line "use crate::host::descriptor::socket::abstract_unix_ns::AbstractUnixNamespace;"
        --raw-line "type Arc_AtomicRefCell_AbstractUnixNamespace = Arc<AtomicRefCell<AbstractUnixNamespace>>;"
        --raw-line "type HashSet_String = HashSet<String>;"

        # used to generate #[must_use] annotations
        --enable-function-attribute-detection

        ${CMAKE_CURRENT_SOURCE_DIR}/wrapper.h -o ${CMAKE_CURRENT_SOURCE_DIR}/wrapper.rs -- ${LLVM_FLAGS}
    VERBATIM
    MAIN_DEPENDENCY wrapper.h
    IMPLICIT_DEPENDS C wrapper.h)

# A fake target that depends on the wrapper.
add_custom_target(bindings_main_rust DEPENDS wrapper.rs)

# Only re-generate bindings when explicititly requested, so that
# our CI doesn't need to install the heavy bindgen dependency.
set_property(TARGET bindings_main_rust PROPERTY EXCLUDE_FROM_ALL true)
