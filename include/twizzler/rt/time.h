#pragma once

#include "types.h"

#ifdef __cplusplus
extern "C" {
#endif

enum monotonicity {
  NonMonotonic,
  WeakMonotonic,
  StrongMonotonic,
};

extern struct duration twz_rt_get_monotonic_time(void);
extern struct duration twz_rt_get_system_time(void);
#ifdef __cplusplus
}
#endif
