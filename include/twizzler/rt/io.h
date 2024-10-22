#pragma once

#include "types.h"
#include "fd.h"

enum io_error {
  IoError_Success,
  IoError_Other,
  IoError_WouldBlock,
};

struct io_result {
  enum io_error error;
  size_t value;
};

const uint32_t IO_NONBLOCKING = 1;

const uint32_t WHENCE_START = 0;
const uint32_t WHENCE_END = 1;
const uint32_t WHENCE_CURRENT = 2;

extern struct io_result twz_rt_fd_read(descriptor fd, void *buf, size_t len, uint32_t flags);
extern struct io_result twz_rt_fd_write(descriptor fd, const void *buf, size_t len, uint32_t flags);
extern struct io_result twz_rt_fd_seek(descriptor fd, uint32_t whence, int64_t offset);

