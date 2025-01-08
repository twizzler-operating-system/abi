#pragma once

#include "types.h"
#include "handle.h"
#include <stddef.h>

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

/// Given a pointer, find the start of the associated object. The returned pointer and the passed pointer p
/// are guaranteed to be in the same object, allowing pointer arithmetic.
extern void *twz_rt_locate_object_start(void *p);

/// Given a pointer, find the associated object. The returned pointer and the passed pointer p
/// are guaranteed to be in the same object, allowing pointer arithmetic.
extern struct object_handle twz_rt_get_object_handle(void *p);

/// Resolve an FOT entry, returning an object handle for the target object with at least valid_len bytes of
/// addressable memory.
extern struct map_result twz_rt_resolve_fot(struct object_handle *handle, uint64_t idx, size_t valid_len);
/// Does the same as twz_rt_resolve_fot but optimizes for local pointers and avoids cloning handles if possible. Returns null on failure
/// with no error code. Callers should try the twz_rt_resolve_fot function if this one fails.
extern void *twz_rt_resolve_fot_local(void *start, uint64_t idx, size_t valid_len);

/// Insert the given entry into the FOT, or return the existing entry if it already exists in this object's FOT.
extern int64_t twz_rt_insert_fot(struct object_handle *handle, void *entry);

// Not intended for public use.
extern void __twz_rt_map_two_objects(rt_objid id_1, map_flags flags_1, rt_objid id_2, map_flags flags_2, struct map_result *res_1, struct map_result *res_2);
#ifdef __cplusplus
}
#endif
