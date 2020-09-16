/*
 * The Shadow Simulator
 * See LICENSE for licensing information
 */
// clang-format off


#ifndef main_bindings_h
#define main_bindings_h

/* Warning, this file is autogenerated by cbindgen. Don't modify this manually. */

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>
#include "main/host/syscall_types.h"
#include "main/host/thread.h"

// A queue of byte chunks.
typedef struct ByteQueue ByteQueue;

// Manages the address-space for a plugin process.
typedef struct MemoryManager MemoryManager;

void bytequeue_free(ByteQueue *bq_ptr);

ByteQueue *bytequeue_new(size_t chunk_size);

size_t bytequeue_pop(ByteQueue *bq, unsigned char *dst, size_t len);

void bytequeue_push(ByteQueue *bq, const unsigned char *src, size_t len);

// # Safety
// * `mm` must point to a valid object.
void memorymanager_free(MemoryManager *mm);

// Get a mutable pointer to the plugin's memory via mapping, or via the thread APIs.
// # Safety
// * `mm` and `thread` must point to valid objects.
void *memorymanager_getMutablePtr(MemoryManager *memory_manager,
                                  Thread *thread,
                                  PluginPtr plugin_src,
                                  uintptr_t n);

// Get a readable pointer to the plugin's memory via mapping, or via the thread APIs.
// # Safety
// * `mm` and `thread` must point to valid objects.
const void *memorymanager_getReadablePtr(MemoryManager *memory_manager,
                                         Thread *thread,
                                         PluginPtr plugin_src,
                                         uintptr_t n);

// Get a writeagble pointer to the plugin's memory via mapping, or via the thread APIs.
// # Safety
// * `mm` and `thread` must point to valid objects.
void *memorymanager_getWriteablePtr(MemoryManager *memory_manager,
                                    Thread *thread,
                                    PluginPtr plugin_src,
                                    uintptr_t n);

// Fully handles the `brk` syscall, keeping the "heap" mapped in our shared mem file.
SysCallReg memorymanager_handleBrk(MemoryManager *memory_manager,
                                   Thread *thread,
                                   PluginPtr plugin_src);

// Fully handles the `mmap` syscall
SysCallReg memorymanager_handleMmap(MemoryManager *memory_manager,
                                    Thread *thread,
                                    PluginPtr addr,
                                    uintptr_t len,
                                    int32_t prot,
                                    int32_t flags,
                                    int32_t fd,
                                    int64_t offset);

SysCallReg memorymanager_handleMprotect(MemoryManager *memory_manager,
                                        Thread *thread,
                                        PluginPtr addr,
                                        uintptr_t size,
                                        int32_t prot);

SysCallReg memorymanager_handleMremap(MemoryManager *memory_manager,
                                      Thread *thread,
                                      PluginPtr old_addr,
                                      uintptr_t old_size,
                                      uintptr_t new_size,
                                      int32_t flags,
                                      PluginPtr new_addr);

// Fully handles the `munmap` syscall
SysCallReg memorymanager_handleMunmap(MemoryManager *memory_manager,
                                      Thread *thread,
                                      PluginPtr addr,
                                      uintptr_t len);

// # Safety
// * `thread` must point to a valid object.
MemoryManager *memorymanager_new(Thread *thread);

void rust_logging_init(void);

#endif /* main_bindings_h */
