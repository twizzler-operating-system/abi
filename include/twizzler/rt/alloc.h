#pragma once

#include <stdint.h>
#include <stddef.h>

#ifdef __cplusplus
extern "C" {
#endif

typedef uint32_t alloc_flags;

const alloc_flags ZERO_MEMORY = 1;

extern void *twz_rt_malloc(size_t sz, size_t align, alloc_flags flags);
extern void *twz_rt_dealloc(void *ptr, size_t sz, size_t align, alloc_flags flags);
extern void *twz_rt_realloc(void *ptr, size_t sz, size_t align, size_t new_size, alloc_flags flags);

#ifdef __cplusplus
}
#endif
