#pragma once

#include "types.h"
#include "handle.h"

#ifdef __cplusplus
extern "C" {
#endif

/// Possible mapping errors
enum map_error {
  MapError_Success,
  MapError_Other,
  MapError_OutOfResources,
  MapError_NoSuchObject,
  MapError_PermissionDenied,
  MapError_InvalidArgument,
};

/// Result map_object call
struct map_result {
  /// Handle, if error is set to Success.
  struct object_handle handle;
  enum map_error error;
};

/// Map with READ permission.
const map_flags MAP_FLAG_R = 1;
/// Map with WRITE permission.
const map_flags MAP_FLAG_W = 2;
/// Map with EXEC permission.
const map_flags MAP_FLAG_X = 4;

/// Map an object with a given ID and flags.
extern struct map_result twz_rt_map_object(rt_objid id, map_flags flags);
/// Release an object handle. After calling this, the handle may not be used.
extern void twz_rt_release_handle(struct object_handle *handle);

// Not intended for public use.
extern void __twz_rt_map_two_objects(rt_objid id_1, map_flags flags_1, rt_objid id_2, map_flags flags_2, struct map_result *res_1, struct map_result *res_2);
#ifdef __cplusplus
}
#endif
