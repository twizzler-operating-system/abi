#pragma once

#include <stdint.h>
#include "handle.h"

#ifdef __cplusplus
extern "C" {
#endif

struct dl_phdr_info {
	uintptr_t dlpi_addr;
	const char *dlpi_name;
	const void *dlpi_phdr;
  uint32_t dlpi_phnum;
	unsigned long long int dlpi_adds;
	unsigned long long int dlpi_subs;
	size_t dlpi_tls_modid;
	void *dlpi_tls_data;
};

struct dso {
  uint32_t id;
  struct object_handle image_handle;
  const void *image_start;
  size_t image_len;
  struct dl_phdr_info dl_info;
};

extern bool twz_rt_get_dso(uint32_t id, struct dso *dso);
extern int twz_rt_iter_phdr(int (*cb)(const struct dl_phdr_info *));

const uint32_t TWZ_RT_EXEID = 0;

#ifdef __cplusplus
}
#endif
