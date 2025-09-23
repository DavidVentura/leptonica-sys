extern crate bindgen;
extern crate cmake;

use std::env;
use std::path::PathBuf;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let target = env::var("TARGET").unwrap();

    let _dst = cmake::Config::new("leptonica")
        .define("ANDROID_BUILD", "ON")
        .define("BUILD_PROG", "OFF")
        .define("BUILD_SHARED_LIBS", "OFF")
        .define("ENABLE_ZLIB", "OFF")
        .define("ENABLE_PNG", "OFF")
        .define("ENABLE_GIF", "OFF")
        .define("ENABLE_JPEG", "OFF")
        .define("ENABLE_TIFF", "OFF")
        .define("ENABLE_WEBP", "OFF")
        .define("ENABLE_OPENJPEG", "OFF")
        .define("CMAKE_INSTALL_PREFIX", &out_dir)
        .define("CMAKE_C_FLAGS", "-DMINIMUM_SEVERITY=6")
        .out_dir(&format!("{}/leptonica-build-{}", out_dir, target))
        .always_configure(true)
        .build();

    let lib_path = format!("{}/lib", out_dir);
    println!("cargo:rustc-link-search=native={}", lib_path);
    println!("cargo:rustc-link-lib=static=leptonica");

    let include_path = PathBuf::from(&format!("{}/include", out_dir));

    let mut bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg(format!("-I{}", include_path.display()));

    bindings = bindings
        .blocklist_type("max_align_t")
        // u128
        .blocklist_function("qecvt_r")
        .blocklist_function("qfcvt_r")
        .blocklist_function("qecvt")
        .blocklist_function("qfcvt")
        .blocklist_function("qgcvt")
        .blocklist_function("strtold")
        .blocklist_function("strtold_l")
        // [u64; 4usize]
        .blocklist_function("vasprintf")
        .blocklist_function("vdprintf")
        .blocklist_function("vfprintf")
        .blocklist_function("vfscanf")
        .blocklist_function("vfscanf1")
        .blocklist_function("vprintf")
        .blocklist_function("vscanf")
        .blocklist_function("vscanf1")
        .blocklist_function("vsnprintf")
        .blocklist_function("vsprintf")
        .blocklist_function("vsscanf")
        .blocklist_function("vsscanf1");

    let bindings = bindings
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // Expose paths for dependent crates (becomes DEP_LEPT_* environment variables)
    println!("cargo:include={}", include_path.display());
    println!("cargo:lib={}", lib_path);
}
