// build.rs
fn main() {
    println!("cargo:rustc-link-lib=dylib=stdc++"); // This line may be unnecessary for some environment.
    println!("cargo:rustc-link-search=libvirtd");
}
