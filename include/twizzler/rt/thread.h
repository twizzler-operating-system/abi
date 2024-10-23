#pragma once

#include<stdint.h>
#include"types.h"
#include<stdbool.h>
#include<stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef uint32_t futex_word;

extern bool futex_wait(_Atomic futex_word *ptr, futex_word expected, struct option_duration timeout);
extern bool futex_wake_one(_Atomic futex_word *ptr);
extern void futex_wake_all(_Atomic futex_word *ptr);

extern void yield_now(void);
extern void set_name(const char *name, size_t len);
extern void sleep(struct duration dur);

struct tls_index {
  size_t mod_id;
  size_t offset;
};

extern void *tls_get_addr(struct tls_index index);

typedef uint32_t thread_id;

struct spawn_args {
  size_t stack_size;
  uintptr_t start;
  size_t arg;
};

enum spawn_error {
  Spawn_Success,
  Spawn_Other,
  Spawn_InvalidArgument,
  Spawn_ObjectNotFound,
  Spawn_PermissionDenied,
  Spawn_KernelError,
};

struct spawn_result {
  thread_id id;
  enum spawn_error err;
};

extern struct spawn_result spawn(struct spawn_args args);

enum join_result {
  Join_Success,
  Join_ThreadNotFound,
  Join_Timeout,
};

extern enum join_result join(thread_id id, struct option_duration timeout);
#ifdef __cplusplus
}
#endif
