#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[allow(unused_imports)]
use libc::*;

// bindgen wrapper.h -o src/bindings.rs --whitelist-function '^nvds_.*' --whitelist-function '^gst_.*_nvds_.*' --whitelist-var '^NvDs.*' --whitelist-type '^NvDs.*' --whitelist-type '^NvBuf.*' -- $(pkg-config --cflags gstreamer-1.0) -I/opt/nvidia/deepstream/deepstream/sources/includes/
include!("./bindings.rs");

// ---------------------------------------------------------------------------
// NvBufSurface functions.
//
// bindgen's whitelist above binds the NvBuf* *types* but not the NvBufSurface*
// *functions* (the function allowlist is `nvds_*` only). These declarations
// reference the already-generated structs, so no struct is mirrored by hand.
// TODO: fold `--allowlist-function 'NvBufSurface.*'` into the bindgen command
// and regenerate bindings.rs (do it on an x86 DeepStream box; wrapper.h pulls
// DeepStream headers that aren't present on Jetson), then drop this block.
// ---------------------------------------------------------------------------
extern "C" {
    pub fn NvBufSurfaceAllocate(
        surf: *mut *mut NvBufSurface,
        batch_size: c_uint,
        params: *mut NvBufSurfaceAllocateParams,
    ) -> c_int;
    pub fn NvBufSurfaceDestroy(surf: *mut NvBufSurface) -> c_int;
    pub fn NvBufSurfaceMap(
        surf: *mut NvBufSurface,
        index: c_int,
        plane: c_int,
        type_: NvBufSurfaceMemMapFlags,
    ) -> c_int;
    pub fn NvBufSurfaceUnMap(surf: *mut NvBufSurface, index: c_int, plane: c_int) -> c_int;
    pub fn NvBufSurfaceSyncForDevice(surf: *mut NvBufSurface, index: c_int, plane: c_int) -> c_int;
}

// ---------------------------------------------------------------------------
// Small NVMM allocation helpers, in pure Rust over the bindings above. These
// replace the earlier C shim: the opaque `*mut c_void` handle is an
// `NvBufSurface*`; pass it back to `nvbuf_free`. All are `unsafe` (raw FFI).
// ---------------------------------------------------------------------------

/// Allocate `count` NV12 block-linear NVMM surfaces (the layout NVDEC decodes
/// into). Fills `out_fds[0..count]` with each surface's dmabuf fd. Returns an
/// opaque `NvBufSurface*` handle (pass to [`nvbuf_free`]), or null on failure.
pub unsafe fn nvbuf_alloc_nv12(
    w: c_uint,
    h: c_uint,
    count: c_uint,
    out_fds: *mut c_int,
) -> *mut c_void {
    let mut p: NvBufSurfaceAllocateParams = std::mem::zeroed();
    p.params.width = w;
    p.params.height = h;
    p.params.colorFormat = NvBufSurfaceColorFormat_NVBUF_COLOR_FORMAT_NV12;
    p.params.layout = NvBufSurfaceLayout_NVBUF_LAYOUT_BLOCK_LINEAR;
    p.params.memType = NvBufSurfaceMemType_NVBUF_MEM_SURFACE_ARRAY;
    p.memtag = NvBufSurfaceTag_NvBufSurfaceTag_VIDEO_DEC;

    let mut surf: *mut NvBufSurface = std::ptr::null_mut();
    if NvBufSurfaceAllocate(&mut surf, count, &mut p) != 0 || surf.is_null() {
        return std::ptr::null_mut();
    }
    (*surf).numFilled = count;
    let list = (*surf).surfaceList;
    for i in 0..count as isize {
        *out_fds.offset(i) = (*list.offset(i)).bufferDesc as c_int;
    }
    surf as *mut c_void
}

/// Allocate ONE raw NVMM buffer of `size` bytes (e.g. a coded-input plane
/// buffer). Returns an opaque handle; `*out_fd` receives its dmabuf fd. Null on
/// failure.
pub unsafe fn nvbuf_alloc_raw(size: c_uint, out_fd: *mut c_int) -> *mut c_void {
    let mut p: NvBufSurfaceAllocateParams = std::mem::zeroed();
    p.params.memType = NvBufSurfaceMemType_NVBUF_MEM_HANDLE;
    p.params.size = size; // raw byte buffer; width/height/format ignored
    p.memtag = NvBufSurfaceTag_NvBufSurfaceTag_VIDEO_DEC;

    let mut surf: *mut NvBufSurface = std::ptr::null_mut();
    if NvBufSurfaceAllocate(&mut surf, 1, &mut p) != 0 || surf.is_null() {
        return std::ptr::null_mut();
    }
    (*surf).numFilled = 1;
    *out_fd = (*(*surf).surfaceList).bufferDesc as c_int;
    surf as *mut c_void
}

/// Map surface 0 / plane 0 of `handle` for CPU read/write; returns the CPU
/// pointer (or null on failure).
pub unsafe fn nvbuf_map_write(handle: *mut c_void) -> *mut u8 {
    let surf = handle as *mut NvBufSurface;
    if NvBufSurfaceMap(surf, 0, 0, NvBufSurfaceMemMapFlags_NVBUF_MAP_READ_WRITE) != 0 {
        return std::ptr::null_mut();
    }
    (*(*surf).surfaceList).mappedAddr.addr[0] as *mut u8
}

/// Flush CPU writes so the hardware sees them (call before QBUF).
pub unsafe fn nvbuf_sync_for_device(handle: *mut c_void) {
    NvBufSurfaceSyncForDevice(handle as *mut NvBufSurface, 0, 0);
}

/// Free a handle from [`nvbuf_alloc_nv12`] or [`nvbuf_alloc_raw`].
pub unsafe fn nvbuf_free(handle: *mut c_void) {
    if !handle.is_null() {
        NvBufSurfaceDestroy(handle as *mut NvBufSurface);
    }
}
