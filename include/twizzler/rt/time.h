#pragma once

#include "types.h"

enum monotonicity {
  NonMonotonic,
  WeakMonotonic,
  StrongMonotonic,
};

extern struct duration twz_rt_get_monotonic_time(void);
extern struct duration twz_rt_get_system_time(void);
