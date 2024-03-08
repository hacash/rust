extern crate cc;

// for x16rs

fn main() {
    cc::Build::new()
        .file("src/x16rs/x16rs.c")
        .warnings(false)
        .flag("-Wunused-but-set-variable")
        .compile("libx16rs.so");
    println!("cargo:rerun-if-changed=src/x16rs/x16rs.c");
}