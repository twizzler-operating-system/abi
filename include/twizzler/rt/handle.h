#pragma once

#include "types.h"

struct object_handle {
  rt_objid id;
  void *runtime_info;
  void *start;
  void *meta;
  uint32_t map_flags;
  uint32_t _resv;
};
