fn main() {
        println!("cargo:rustc-link-search=native=/usr/lib/aarch64-linux-gnu/tegra");
        println!("cargo:rustc-link-lib=dylib=nvbufsurface");
        println!("cargo:rustc-link-lib=dylib=nvbufsurftransform");
        println!("cargo:rustc-link-lib=dylib=nvscicommon");
}
