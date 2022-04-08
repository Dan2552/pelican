#[link(name = "CoreFoundation", kind = "framework")]
fn main() {
    #[cfg(target_os = "macos")]
    objective_c();
}

#[cfg(target_os = "macos")]
fn objective_c() {
    println!("cargo:rustc-link-lib=framework=Foundation");
    cc::Build::new()
        .file("src/objc/objc.m")
        .compile("objc");
}
