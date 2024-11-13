#pragma once

#include "types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef int32_t descriptor;

struct open_info {
  const char *name;
  size_t len;
};

enum open_error {
  OpenError_Success,
  OpenError_Other,
  OpenError_LookupFail,
  OpenError_PermissionDenied,
  OpenError_InvalidArgument,
};

struct open_result {
  descriptor fd;
  enum open_error error;
};

extern struct open_result twz_rt_fd_open(struct open_info info);
extern void twz_rt_fd_close(descriptor fd);

typedef uint32_t fd_flags;
const fd_flags FD_IS_TERMINAL = 1;
struct fd_info {
  fd_flags flags;
};

extern bool twz_rt_fd_get_info(descriptor fd, struct fd_info *info);

typedef uint32_t fd_cmd;
const fd_cmd FD_CMD_DUP = 1;
typedef uint32_t fd_cmd_err;
const fd_cmd_err FD_CMD_SUCCESS = 0;

extern fd_cmd_err twz_rt_fd_cmd(descriptor fd, fd_cmd cmd, void *arg, void *ret);

#ifdef __cplusplus
}
#endif
