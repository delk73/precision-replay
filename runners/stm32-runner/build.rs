fn main() {
    println!(
        "cargo:rustc-link-search={}",
        std::env::var("CARGO_MANIFEST_DIR").unwrap()
    );
    println!("cargo:rustc-link-arg-bin=stm32-runner=-Tmemory.x");
}
