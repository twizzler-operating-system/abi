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

struct comp_init_info {
  void (*legacy_init)();
  void (**init_array)();
  size_t init_array_len;
  void *comp_config_info;
};

struct minimal_init_info {
  char **args;
  size_t argc;
  char **envp;
  void *phdrs;
  size_t nr_phdrs;
};

union init_info_ptrs {
    struct comp_init_info *comp;
    struct minimal_init_info *min;
    void *monitor;
};

struct runtime_info {
  int32_t flags;
  int32_t kind;
  union init_info_ptrs init_info;
};

const int32_t RUNTIME_INIT_MIN = 0;
const int32_t RUNTIME_INIT_MONITOR = 1;
const int32_t RUNTIME_INIT_COMP = 2;

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
