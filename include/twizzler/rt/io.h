#pragma once

#include "types.h"
#include "fd.h"

#ifdef __cplusplus
extern "C" {
#endif

/// Possible IO errors
enum io_error {
  /// Success
  IoError_Success,
  /// Other error
  IoError_Other,
  /// Error during seek
  IoError_SeekError,
  /// Invalid descriptor
  IoError_InvalidDesc,
  /// Operation would block, but nonblocking behavior was specified.
  IoError_WouldBlock,
};

/// Result of IO operations
struct io_result {
  /// Error value, or success.
  enum io_error error;
  /// Returned value, only valid if error is set to Success.
  size_t value;
};

/// Type of whence values for seek.
typedef uint32_t whence;

/// Flags for IO operations
typedef uint32_t io_flags;

/// Non-blocking behavior specified. If the operation would block, return io_result with error set to WouldBlock instead.
const io_flags IO_NONBLOCKING = 1;

/// Seek offset from start of file
const whence WHENCE_START = 0;
/// Seek offset from end of file
const whence WHENCE_END = 1;
/// Seek offset from current fd position
const whence WHENCE_CURRENT = 2;

/// Optional offset. If value is FD_POS, use the file descriptor position.
typedef int64_t optional_offset;
const optional_offset FD_POS = -1;

/// Read from a file. May read less than specified len.
extern struct io_result twz_rt_fd_pread(descriptor fd, optional_offset offset, void *buf, size_t len, io_flags flags);
/// Write to a file. May write less than specified len.
extern struct io_result twz_rt_fd_pwrite(descriptor fd, optional_offset offset, const void *buf, size_t len, io_flags flags);
/// Seek to a specified point in the file.
extern struct io_result twz_rt_fd_seek(descriptor fd, whence whence, int64_t offset);

/// Io vec, a buffer and a len.
struct io_vec {
  /// Pointer to buffer.
  char *buf;
  /// Length of buffer in bytes.
  size_t len;
};

/// Do vectored IO read.
extern struct io_result twz_rt_fd_preadv(descriptor fd, optional_offset offset, const struct io_vec *iovs, size_t nr_iovs, io_flags flags);
/// Do vectored IO write.
extern struct io_result twz_rt_fd_pwritev(descriptor fd, optional_offset offset, const struct io_vec *iovs, size_t nr_iovs, io_flags flags);
#ifdef __cplusplus
}
#endif
