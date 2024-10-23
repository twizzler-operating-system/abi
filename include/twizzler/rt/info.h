#pragma once

#include "time.h"

#ifdef __cplusplus
extern "C" {
#endif

struct system_info {
  enum monotonicity clock_monotonicity;
  size_t available_parallelism;
};

extern struct system_info twz_rt_get_sysinfo(void);
#ifdef __cplusplus
}
#endif
