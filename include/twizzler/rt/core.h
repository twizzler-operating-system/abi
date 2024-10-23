#pragma once

#include "types.h"

#ifdef __cplusplus
extern "C" {
#endif

struct basic_aux {
    size_t argc;
    char **args;
    char **env;
};

struct basic_return {
  int32_t code;
};

struct runtime_info {
  int32_t flags;
};

_Noreturn void twz_rt_exit(int32_t code);
_Noreturn void twz_rt_abort(void);
struct option_i32 twz_rt_pre_main_hook(void);
void twz_rt_post_main_hook(void);
_Noreturn void twz_rt_runtime_entry(const struct runtime_info *arg, struct basic_return (*std_entry)(struct basic_aux));
#ifdef __cplusplus
}
#endif
