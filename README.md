# Linux AIO Bindings Generator with SGX OCall Support

This project provides a `build.rs` script to generate **Rust FFI bindings specifically for Linux AIO (Asynchronous I/O) system calls**, with **Intel SGX support for OCalls**.

## Usage

- By default, the build script does nothing.
- To generate bindings for AIO and related system calls, build with the `bindgen` feature:

```bash
cargo build --features bindgen
```

- To overwrite bindings into the repository (e.g., src/sys/mod.rs), also enable the overwrite feature:

```bash
cargo build --features "bindgen overwrite"
