use std::*;
use std::path::*;
use std::sync::{Arc};

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
mod vm;
mod chain;
mod node;
mod server;
mod tests;

use crate::sys::*;
use crate::base::field::Hash;
use crate::interface::field::*;
use crate::interface::chain::*;
use crate::core::account::Account;
use crate::mint::checker::*;
use crate::chain::engine::*;
use crate::node::node::*;
use crate::server::*;

use crate::tests::*;

/**

sudo apt install g++
sudo apt install cmake     


RUSTFLAGS="$RUSTFLAGS -Awarnings" RUST_BACKTRACE=1 cargo check / build / run
mkdir -p ./target/debug/ && cp hacash.config.ini ./target/debug/ && RUSTFLAGS="$RUSTFLAGS -Awarnings" RUST_BACKTRACE=1 cargo run
rm -rf ./target/debug/ && cargo clean

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
    let mint_checker = Box::new(BlockMintChecker::new(&iniobj));

    // engine
    let engine = BlockEngine::open(&iniobj, mint_checker);
    let engptr: Arc<BlockEngine> = Arc::new(engine);

    // node
    let mut hnode = Arc::new(HacashNode::open(&iniobj, engptr.clone()));

    // server
    let server = DataServer::open(&iniobj, engptr.clone());
    std::thread::spawn(move||{
        server.start(); // http rpc 
    });

    // handle ctr+c to close
    let hn2 = hnode.clone();
    ctrlc::set_handler( move || {
        hn2.close();
    });

    // start
    HacashNode::start(hnode);

    // on closed
    println!("\nHacash node closed.");


    // test
    // engine_test_3(engptr);


    // run 10 year
    // println!("main run 10 year");
    // thread::sleep(std::time::Duration::from_secs(60*60*24*365*10));
}










