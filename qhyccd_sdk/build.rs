
fn main() {
    // Tell cargo to link the precompiled QHYCCD library
    println!("cargo:rustc-link-search=/usr/local/lib/libqhyccd");
    println!("cargo:rustc-link-lib=qhyccd");
}
