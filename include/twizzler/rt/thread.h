#pragma once

#include<stdint.h>
#include"types.h"
#include<stdbool.h>
#include<stddef.h>

extern bool futex_wait(_Atomic uint32_t *ptr, uint32_t expected, struct option_duration timeout);
extern bool futex_wake_one(_Atomic uint32_t *ptr);
extern void futex_wake_all(_Atomic uint32_t *ptr);

extern void yield_now(void);
extern void set_name(const char *name);
extern void sleep(struct duration dur);

struct tls_index {
  size_t mod_id;
  size_t offset;
};

extern void *tls_get_addr(struct tls_index index);

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
  uint32_t id;
  enum spawn_error err;
};

extern struct spawn_result spawn(struct spawn_args args);

enum join_result {
  Join_Success,
  Join_ThreadNotFound,
  Join_Timeout,
};

extern enum join_result join(uint32_t id, struct option_duration timeout);
