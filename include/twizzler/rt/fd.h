#pragma once

#include "types.h"

#ifdef __cplusplus
extern "C" {
#endif

/// An open descriptor for a runtime file handle.
typedef int32_t descriptor;

/// Information for opening a file.
struct open_info {
  /// File name pointer.
  const char *name;
  /// Length of file name in bytes.
  size_t len;
};

/// Possible open error conditions.
enum open_error {
  /// Open success.
  OpenError_Success,
  /// Other error.
  OpenError_Other,
  /// Failed to lookup provided file name.
  OpenError_LookupFail,
  /// Permission denied.
  OpenError_PermissionDenied,
  /// Argument was invalid.
  OpenError_InvalidArgument,
};

/// Result of open call.
struct open_result {
  /// If error is Success, this contains a valid descriptor.
  descriptor fd;
  /// Error code, or success.
  enum open_error error;
};

/// Open a file.
extern struct open_result twz_rt_fd_open(struct open_info info);

/// Close a file descriptor. If the file descriptor is invalid
/// or already closed, this function does nothing.
extern void twz_rt_fd_close(descriptor fd);

/// Flags a descriptor can have.
typedef uint32_t fd_flags;

/// This file descriptor is a terminal.
const fd_flags FD_IS_TERMINAL = 1;

/// Information about a file descriptor.
struct fd_info {
  /// Flags for the descriptor.
  fd_flags flags;
};

/// Get information about a descriptor. If this returns true, the fd was valid
/// and the data pointed to by info is filled with fd_info data.
extern bool twz_rt_fd_get_info(descriptor fd, struct fd_info *info);

/// Commands for descriptors.
typedef uint32_t fd_cmd;

/// Duplicate this descriptor. The arg argument is ignored. The ret argument points to a descriptor.
const fd_cmd FD_CMD_DUP = 1;
/// Sync the underlying storage of the file descriptor.
const fd_cmd FD_CMD_SYNC = 1;
/// Delete the underlying object.
const fd_cmd FD_CMD_DELETE = 1;

/// Errors for twz_rt_fd_cmd.
typedef uint32_t fd_cmd_err;

/// Success value for twz_rt_fd_cmd.
const fd_cmd_err FD_CMD_SUCCESS = 0;

/// Perform a command on the descriptor. The arguments arg and ret are interpreted according to
/// the command specified.
extern fd_cmd_err twz_rt_fd_cmd(descriptor fd, fd_cmd cmd, void *arg, void *ret);

#ifdef __cplusplus
}
#endif
