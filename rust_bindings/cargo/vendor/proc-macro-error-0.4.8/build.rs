#[rustversion::not(nightly)]
fn check() {
    println!("cargo:rustc-cfg=use_fallback");
}

#[rustversion::nightly]
fn check() {}

fn main() {
    check()
}
