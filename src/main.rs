use std::*;
use std::path::*;
use std::sync::{Arc};

#[macro_use]
extern crate ini;


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
mod vm;
mod chain;
mod node;
mod tests;

use crate::sys::*;
use crate::base::field::Hash;
use crate::interface::field::*;
use crate::core::account::Account;
use crate::mint::checker::*;
use crate::chain::engine::*;
use crate::node::node::*;

use crate::tests::*;

/**

sudo apt install g++
sudo apt install cmake     

RUSTFLAGS="$RUSTFLAGS -Awarnings" RUST_BACKTRACE=1 cargo check / build / run
cp hacash.config.ini ./target/debug/ && RUSTFLAGS="$RUSTFLAGS -Awarnings" RUST_BACKTRACE=1 cargo run


*/


fn main() {
    
    // delete datadir
    // std::fs::remove_dir_all("./hacash_mainnet_data");

    // main_test8327459283();
    // main_test_vecspeed387425983();

    let inicnf = config::load_config();
    // deal datadir
    start_hacash_node(inicnf);

}



/**
 * create and start hash node
 */
fn start_hacash_node(iniobj: sys::IniObj) {
    // println!("startHacashNode ini={:?}", iniobj);
    // mint
    let mint_checker = Box::new(BlockMintChecker::create());

    // engine
    let engine = BlockEngine::open(&iniobj, mint_checker);
    let engptr = Arc::new(engine);

    // node
    let mut hnode = HacashNode::open(&iniobj, engptr.clone());
    if let Err(e) = hnode.start() {
        println!("start hacash node error: {}", e);
    }




    // test
    // engine_test_3(engptr);



}










