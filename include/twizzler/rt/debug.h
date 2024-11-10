#pragma once

#include <stdint.h>
#include <stddef.h>
#include "handle.h"

#ifdef __cplusplus
extern "C" {
#endif

struct dl_phdr_info {
	uintptr_t addr;
	const char *name;
	const void *phdr;
  uint32_t phnum;
	unsigned long long int adds;
	unsigned long long int subs;
	size_t tls_modid;
	void *tls_data;
};

typedef uint32_t loaded_image_id;

struct loaded_image {
  struct object_handle image_handle;
  const void *image_start;
  size_t image_len;
  struct dl_phdr_info dl_info;
  loaded_image_id id;
};

extern bool twz_rt_get_loaded_image(loaded_image_id id, struct loaded_image *li);
extern int twz_rt_iter_phdr(int (*cb)(const struct dl_phdr_info *, size_t size, void *data), void *data);

const loaded_image_id TWZ_RT_EXEID = 0;

#ifdef __cplusplus
}
#endif
