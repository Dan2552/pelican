fn main() {
    let target_os = std::env::var("CARGO_CFG_TARGET_OS").unwrap();
    if target_os == "macos" {
        objective_c();
    }
}

fn objective_c() {
    println!("cargo:rustc-link-lib=framework=Foundation");
    cc::Build::new()
        .file("src/objc/objc.m")
        .compile("objc");
}
