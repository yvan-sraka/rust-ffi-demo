extern crate cc;

fn main() {
    cc::Build::new().file("hello.c").compile("libhello.a");
    println!("cargo:rustc-link-search=.");
}
