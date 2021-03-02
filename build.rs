use std::env;

pub fn main() {
    let src_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-lib=static=ffmpeg-wasm");
    println!("cargo:rustc-link-search=native={}", src_dir);
}
