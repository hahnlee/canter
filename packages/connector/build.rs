use cc;

fn main() {
    println!("cargo:rustc-link-search=framework=/Library/Apple/System/Library/PrivateFrameworks");
    println!("cargo:rustc-link-lib=framework=MobileDevice");
    println!("cargo:rustc-link-lib=framework=CoreFoundation");

    cc::Build::new()
        .file("src/c_api/lib.c")
        .compile("c_api");
}
