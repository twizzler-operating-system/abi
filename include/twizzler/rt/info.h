#pragma once

#include "time.h"
struct system_info {
  enum monotonicity clock_monotonicity;
  uint64_t available_parallelism;
};

extern struct system_info twz_rt_get_sysinfo(void);
