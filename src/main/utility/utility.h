/*
 * The Shadow Simulator
 * Copyright (c) 2010-2011, Rob Jansen
 * See LICENSE for licensing information
 */

#ifndef SHD_UTILITY_H_
#define SHD_UTILITY_H_

#include <glib.h>
#include <stdio.h>
#include <netinet/in.h>

#include "main/core/support/definitions.h"

#ifdef DEBUG
#define utility_assert(expr)                                                                       \
    do {                                                                                           \
        if G_LIKELY (expr) {                                                                       \
            ;                                                                                      \
        } else {                                                                                   \
            utility_handleError(__FILE__, __LINE__, G_STRFUNC, "Assertion failed: %s", #expr);     \
        }                                                                                          \
    } while (0)
#else
#define utility_assert(expr)
#endif

#define utility_panic(...) utility_handleError(__FILE__, __LINE__, G_STRFUNC, __VA_ARGS__);

#ifdef DEBUG
/**
 * Memory magic for assertions that memory has not been freed. The idea behind
 * this approach is to declare a value in each struct using MAGIC_DECLARE,
 * define it using MAGIC_INIT during object creation, and clear it during
 * cleanup using MAGIC_CLEAR. Any time the object is referenced, we can check
 * the magic value using MAGIC_ASSERT. If the assert fails, there is a bug.
 *
 * In general, this should only be used in DEBUG mode. Once we are somewhat
 * convinced on Shadow's stability (for releases), these macros will do nothing.
 *
 * MAGIC_VALUE is an arbitrary value.
 *
 * @todo add #ifdef DEBUG
 */
#define MAGIC_VALUE 0xAABBCCDD

/**
 * Declare a member of a struct to hold a MAGIC_VALUE. This should be placed in
 * the declaration of a struct, generally as the last member of the struct. If
 * the struct needs to have the same size in both debug and release mode, it
 * can use MAGIC_DECLARE_ALWAYS.
 */
#define MAGIC_DECLARE        guint magic
#define MAGIC_DECLARE_ALWAYS guint magic

/**
 * Initialize a value declared with MAGIC_DECLARE to MAGIC_VALUE. This is useful
 * for initializing the magic value in a struct initializer.
 */
#define MAGIC_INITIALIZER .magic = MAGIC_VALUE,

/**
 * Initialize a value declared with MAGIC_DECLARE to MAGIC_VALUE. This is useful
 * for initializing the magic value in an independent statement.
 */
#define MAGIC_INIT(object) (object)->magic = MAGIC_VALUE

/**
 * Assert that a struct declared with MAGIC_DECLARE and initialized with
 * MAGIC_INIT still holds the value MAGIC_VALUE.
 */
#define MAGIC_ASSERT(object)                                                   \
    utility_assert((object) && ((object)->magic == MAGIC_VALUE))

/**
 * CLear a magic value. Future assertions with MAGIC_ASSERT will fail.
 */
#define MAGIC_CLEAR(object) object->magic = 0
#else
#define MAGIC_VALUE
#define MAGIC_DECLARE
#define MAGIC_DECLARE_ALWAYS guint magic
#define MAGIC_INITIALIZER
#define MAGIC_INIT(object)
#define MAGIC_ASSERT(object)
#define MAGIC_CLEAR(object)
#endif

guint utility_ipPortHash(in_addr_t ip, in_port_t port);
guint utility_int16Hash(gconstpointer value);
gboolean utility_int16Equal(gconstpointer value1, gconstpointer value2);
gint utility_doubleCompare(const gdouble* value1, const gdouble* value2, gpointer userData);
gint utility_simulationTimeCompare(const SimulationTime* value1, const SimulationTime* value2,
        gpointer userData);
gchar* utility_getHomePath(const gchar* path);
guint utility_getRawCPUFrequency(const gchar* freqFilename);
gboolean utility_isRandomPath(const gchar* path);

gboolean utility_removeAll(const gchar* path);
gboolean utility_copyAll(const gchar* srcPath, const gchar* dstPath);

GString* utility_getFileContents(const gchar* fileName);
gchar* utility_getNewTemporaryFilename(const gchar* templateStr);
gboolean utility_copyFile(const gchar* fromPath, const gchar* toPath);

gchar* utility_strvToNewStr(gchar** strv);

__attribute__((__format__(__printf__, 4, 5)))
_Noreturn void utility_handleError(const gchar* file, gint line, const gchar* funtcion,
                                   const gchar* message, ...);

/* Converts millis milliseconds to a timespec with the corresponding number
 * of seconds and nanoseconds. */
struct timespec utility_timespecFromMillis(int64_t millis);

/* If a process exited by a signal, use this return code. */
int return_code_for_signal(int signal);

/* Calling anything other than a thin syscall wrapper between `vfork` and `exec`
 can lead to confusing behavior and crashes. Use this function on errors to
 safely abort the process (with a core dump, if enabled). */
_Noreturn void die_after_vfork();

#endif /* SHD_UTILITY_H_ */
