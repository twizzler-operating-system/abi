#pragma once

#include "types.h"
#include "handle.h"

#ifdef __cplusplus
extern "C" {
#endif

enum map_error {
  MapError_Success,
  MapError_Other,
  MapError_OutOfResources,
  MapError_NoSuchObject,
  MapError_PermissionDenied,
  MapError_InvalidArgument,
};

struct map_result {
  struct object_handle handle;
  enum map_error error;
};

const map_flags MAP_FLAG_R = 1;
const map_flags MAP_FLAG_W = 2;
const map_flags MAP_FLAG_X = 4;

extern struct map_result twz_rt_map_object(rt_objid id, map_flags flags);
extern void twz_rt_release_handle(struct object_handle *handle);

extern void __twz_rt_map_two_objects(rt_objid id_1, map_flags flags_1, rt_objid id_2, map_flags flags_2, struct map_result *res_1, struct map_result *res_2);
#ifdef __cplusplus
}
#endif
