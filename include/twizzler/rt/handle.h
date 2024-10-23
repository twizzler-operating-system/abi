#pragma once

#include "types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef uint32_t map_flags;

struct object_handle {
  rt_objid id;
  void *runtime_info;
  void *start;
  void *meta;
  map_flags map_flags;
  uint32_t valid_len;
};

const size_t LEN_MUL = 0x1000;

#ifdef __cplusplus
}
#endif
