#pragma once
#include<stddef.h>
#include<stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

struct rt_slice {
  size_t len;
  void *ptr;
};

typedef __uint128_t rt_objid;

struct duration {
  uint64_t seconds;
  uint32_t nanos;
};

struct option_i32 {
  int32_t is_some;
  int32_t value;
};

struct option_duration {
  struct duration dur;
  int32_t is_some;
};
#ifdef __cplusplus
}
#endif
