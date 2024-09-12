
#[macro_use]
extern crate ini;
#[macro_use]
extern crate lazy_static; 

mod x16rs;

#[macro_use]
mod sys;
#[macro_use]
mod base;
mod config;
mod interface;
#[macro_use]
mod core;
#[macro_use]
mod protocol;
mod mint;
#[macro_use]
mod vm;
mod chain;
mod node;
mod server;
mod tests;

pub mod run;

use crate::run::*;


fn main() {

    // poworker(); // HAC PoW Miner Worker
    // diaworker(); // Diamond Miner Worker
    fullnode(); // Hacash Full Node

}





/*

## Deal error: linker `cc` not found
sudo apt install build-essential          # with g++ & cmake
sudo apt install pkg-config
sudo apt install openssl
sudo apt install libssl-dev  

## check and build

RUSTFLAGS="$RUSTFLAGS -Awarnings" RUST_BACKTRACE=1 cargo check / build / run
mkdir -p ./target/debug/ && cp hacash.config.ini ./target/debug/ && RUSTFLAGS="$RUSTFLAGS -Awarnings" RUST_BACKTRACE=1 cargo run
RUSTFLAGS="$RUSTFLAGS -Awarnings" RUST_BACKTRACE=1 cargo build --release && cp ./target/release/hacash ./hacash_release && ./hacash_release
rm -rf ./target/debug/ && cargo clean

*/











/*

    delete datadir
    std::fs::remove_dir_all("./hacash_mainnet_data");

    main_test8327459283();
    main_test_vecspeed387425983();
    main_test28374659823746892();
    return;
    return main_test28374659823746892();
    

    // test & examples
    let args: Vec<_> = std::env::args().collect();
    // println!("{:?}", args);
    let testmark = match args.len() >= 2 {
        true => args[1].clone(),
        _ => s!(""),
    };
    match testmark.as_str() {
        "--test" => return rpc_test(),
        "--miner_worker" => return miner_worker(),
        _ => (),
    };
    
*/



