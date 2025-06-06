#pragma once
#include<stddef.h>
#include<stdint.h>

typedef int64_t ssize_t;

#ifdef __cplusplus
extern "C" {
#endif

/// Object ID
typedef __uint128_t objid;

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

const struct option_duration NO_DURATION = { .dur = {.seconds = 0, .nanos = 0}, .is_some = 0};

#ifdef __cplusplus
}
#endif
