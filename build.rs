extern crate cc;

/**

# Step 1: create libx16rs.a
# Step 2: build and run

gcc -c src/x16rs/x16rs.c && ar rcs libx16rs.a x16rs.o && mv *.a ./src/x16rs && rm -f *.o

RUSTFLAGS="$RUSTFLAGS -Awarnings -L ./src/x16rs/" cargo run

# Build static release software
cargo build --release --target=x86_64-unknown-linux-musl
ldd target/x86_64-unknown-linux-musl/release/hacash




*/



fn main() {
    cc::Build::new()
        .file("src/x16rs/x16rs.c")
        .compile("x16rs");
    println!("cargo:rerun-if-changed=src/x16rs/x16rs.c");
}





/*

use std::process::Command;
use std::env;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    panic!("{}", out_dir);

    Command::new("gcc").args(&["src/x16rs/x16rs.c", "-c", "-fPIC", "-o"])
                       .arg(&format!("{}/x16rs.o", out_dir))
                       .status().unwrap();
    Command::new("ar").args(&["crus", "libx16rs.a", "x16rs.o"])
                      .current_dir(&Path::new(&out_dir))
                      .status().unwrap();

    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=x16rs");
    println!("cargo:rerun-if-changed=src/x16rs/x16rs.c");
}

*/