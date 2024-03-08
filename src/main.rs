mod x16rs;

#[macro_use]
mod sys;
#[macro_use]
mod base;
mod interface;
mod core;

use crate::interface::field::*;
use core::account::Account;

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

    let addrhac = core::field::AddrHac::new();

    println!("{} {} {}", "abc 123", s!("error"), addrhac.amount);

    let rshx = x16rs::x16rs_hash(1, &x16rs::calculate_hash(b"123456"));

    println!("{}", hex::encode(rshx));


    let pubkey = hex::decode("817ED5FC625752CBF027A39573E5F40FAC124AC1D983DD91C477C58F2A3BF983F4").unwrap();
    println!("{}", Account::to_readable(&Account::get_address_by_public_key(pubkey.try_into().unwrap())));

    

    // panic_never_call_this!();
}
