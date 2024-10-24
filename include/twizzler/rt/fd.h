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
  OpenError_Sucess,
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
#ifdef __cplusplus
}
#endif
