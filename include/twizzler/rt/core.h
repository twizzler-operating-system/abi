#pragma once

#include "types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef int32_t exit_code;

struct basic_aux {
    size_t argc;
    char **args;
    char **env;
};

struct basic_return {
  exit_code code;
};

struct runtime_info {
  int32_t flags;
};

struct option_exit_code {
  int32_t is_some;
  exit_code value;
};

_Noreturn void twz_rt_exit(exit_code code);
_Noreturn void twz_rt_abort(void);
struct option_exit_code twz_rt_pre_main_hook(void);
void twz_rt_post_main_hook(void);
_Noreturn void twz_rt_runtime_entry(const struct runtime_info *arg, struct basic_return (*std_entry)(struct basic_aux));
#ifdef __cplusplus
}
#endif
