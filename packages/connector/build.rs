fn main() {
    println!("cargo:rustc-link-search=framework=/Library/Apple/System/Library/PrivateFrameworks");
    println!("cargo:rustc-link-lib=framework=MobileDevice");
}
