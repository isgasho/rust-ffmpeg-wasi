# FFMPEG crate for WebAssembly/WASI

This crate bundles FFMPEG's `avcodec` and `avformat` libraries, precompiled for WebAssembly. No native installation required.

Compatible with Fastly's Compute@Edge.

These are *low-level* bindings, directly exposing the original C functions to Rust.

## Usage

```toml
[dependencies]
ffmpeg-wasi = "0"
```
