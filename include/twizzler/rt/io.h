#pragma once

#include "types.h"
#include "fd.h"

#ifdef __cplusplus
extern "C" {
#endif

enum io_error {
  IoError_Success,
  IoError_Other,
  IoError_SeekError,
  IoError_InvalidDesc,
  IoError_WouldBlock,
};

struct io_result {
  enum io_error error;
  size_t value;
};

typedef uint32_t whence;
typedef uint32_t io_flags;

const io_flags IO_NONBLOCKING = 1;

const whence WHENCE_START = 0;
const whence WHENCE_END = 1;
const whence WHENCE_CURRENT = 2;

typedef int64_t optional_offset;
const optional_offset FD_POS = -1;

extern struct io_result twz_rt_fd_pread(descriptor fd, optional_offset offset, void *buf, size_t len, io_flags flags);
extern struct io_result twz_rt_fd_pwrite(descriptor fd, optional_offset offset, const void *buf, size_t len, io_flags flags);
extern struct io_result twz_rt_fd_seek(descriptor fd, whence whence, int64_t offset);

struct io_vec {
  char *buf;
  size_t len;
};

extern struct io_result twz_rt_fd_preadv(descriptor fd, optional_offset offset, const struct io_vec *iovs, size_t nr_iovs, io_flags flags);
extern struct io_result twz_rt_fd_pwritev(descriptor fd, optional_offset offset, const struct io_vec *iovs, size_t nr_iovs, io_flags flags);
#ifdef __cplusplus
}
#endif
