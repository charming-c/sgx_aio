#[cfg(not(feature = "bindgen"))]
fn main() {}

#[cfg(feature = "bindgen")]
fn main() {
    use std::env;
    use std::path::PathBuf;

    const INCLUDE: &str = r#"
#include <time.h>
#include <unistd.h>
#include <linux/aio_abi.h>
#include <sys/eventfd.h>
#include <sys/syscall.h>
    "#;

    #[cfg(not(feature = "overwrite"))]
    let outdir = PathBuf::from(env::var("OUT_DIR").unwrap());

    #[cfg(feature = "overwrite")]
    let outdir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("src/sys");

    bindgen::Builder::default()
        .header_contents("include-file.h", INCLUDE)
        .ctypes_prefix("libc")
        .prepend_enum_name(false)
        .derive_default(true)
        .generate_comments(true)
        .use_core()
        .generate()
        .unwrap()
        .write_to_file(outdir.join("mod.rs"))
        .unwrap();
}
