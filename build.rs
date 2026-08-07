use std::path::Path;

fn main() {
    // Link the NvBufSurface / DeepStream libraries. Target-aware so the crate
    // builds both on Jetson (jetson_multimedia_api, no DeepStream) and on x86
    // DeepStream. No C is compiled here — the bindings come from bindgen and
    // the helpers are pure Rust.
    let is_jetson = Path::new("/usr/src/jetson_multimedia_api/include/nvbufsurface.h").exists();

    if is_jetson {
        println!("cargo:rustc-link-search=native=/usr/lib/aarch64-linux-gnu/tegra");
        println!("cargo:rustc-link-lib=dylib=nvbufsurface");
    } else {
        println!("cargo:rustc-link-search=/opt/nvidia/deepstream/deepstream/lib");
        println!("cargo:rustc-link-lib=nvds_meta");
        println!("cargo:rustc-link-lib=nvdsgst_meta");
        println!("cargo:rustc-link-lib=nvbufsurface");
    }
}
