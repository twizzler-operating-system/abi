#pragma once
#include<stddef.h>
#include<stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

/// Object ID
typedef __uint128_t rt_objid;

/// Duration, containing seconds and nanoseconds.
struct duration {
  uint64_t seconds;
  uint32_t nanos;
};

/// Optional duration.
struct option_duration {
  struct duration dur;
  int32_t is_some;
};

#ifdef __cplusplus
}
#endif
