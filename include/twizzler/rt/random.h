#pragma once

#include <stdint.h>
#include <stddef.h>

typedef uint32_t get_random_flags;

const get_random_flags GET_RANDOM_NON_BLOCKING = 1;

extern size_t twz_rt_get_random(char *buf, size_t len, get_random_flags flags);
