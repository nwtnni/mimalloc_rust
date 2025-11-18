use std::env;

const ROOT: &str = env!("CARGO_MANIFEST_DIR");

fn main() {
    let version = if env::var("CARGO_FEATURE_V3").is_ok() {
        "v3"
    } else {
        "v2"
    };

    println!("cargo:rustc-link-search=native={ROOT}/c_src/mimalloc/{version}/build");
    println!("cargo:rustc-link-lib=static=mimalloc");
}
