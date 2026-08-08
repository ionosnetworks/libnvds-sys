#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[allow(unused_imports)]
use libc::*;

// bindgen wrapper.h -o src/bindings.rs --allowlist-type '^NvBuf.*' --allowlist-function '^NvBuf.*' --allowlist-var '^NVBUF.*' --allowlist-type '^v4l2_ctrl_videodec.*' --allowlist-type '^v4l2_ctrl_video_metadata' --allowlist-type '^v4l2_skip_frames_type' --allowlist-type '^v4l2_videodec.*' --allowlist-var '^V4L2_CID_MPEG_VIDEODEC.*' --allowlist-var '^V4L2_CID_MPEG_VIDEO_SKIP_FRAMES$' --allowlist-var '^V4L2_CID_MPEG_VIDEO_DISABLE_COMPLETE_FRAME_INPUT$' --allowlist-var '^V4L2_DEC_ERROR.*' --allowlist-var '^V4L2_SKIP_FRAMES.*' -- -I/usr/src/jetson_multimedia_api/include
include!("./bindings.rs");
