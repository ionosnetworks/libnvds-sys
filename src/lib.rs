#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

#[allow(unused_imports)]
use libc::*;

// bindgen wrapper.h -o src/bindings.rs --whitelist-function '^nvds_.*' --whitelist-function '^gst_.*_nvds_.*' --whitelist-var '^NvDs.*' --whitelist-type '^NvDs.*' --whitelist-type '^NvBuf.*' -- $(pkg-config --cflags gstreamer-1.0) -I/opt/nvidia/deepstream/deepstream/sources/includes/
include!("./bindings.rs");
