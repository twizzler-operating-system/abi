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
  uint32_t _resv;
};
#ifdef __cplusplus
}
#endif
