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
#[macro_use]
mod vm;
mod chain;
mod node;
mod server;
mod tests;

use crate::sys::*;
use crate::core::field::*;
use crate::base::field::*;
use crate::interface::field::*;
use crate::interface::chain::*;
use crate::core::account::Account;
use crate::mint::checker::*;
use crate::mint::component::*;
use crate::chain::engine::*;
use crate::node::node::*;
use crate::server::*;

use crate::tests::*;

/**

sudo apt install g++
sudo apt install cmake     


RUSTFLAGS="$RUSTFLAGS -Awarnings" RUST_BACKTRACE=1 cargo check / build / run
mkdir -p ./target/debug/ && cp hacash.config.ini ./target/debug/ && RUSTFLAGS="$RUSTFLAGS -Awarnings" RUST_BACKTRACE=1 cargo run
RUSTFLAGS="$RUSTFLAGS -Awarnings" RUST_BACKTRACE=1 cargo build --release && cp ./target/release/hacash ./hacash_release && ./hacash_release
rm -rf ./target/debug/ && cargo clean

*/



const HACASH_NODE_VERSION: &str = "0.1.0";
const HACASH_NODE_BUILD_TIME: &str = "2024.8.1-1";
const HACASH_STATE_DB_UPDT: u32 = 1;



fn main() {
    
    // delete datadir
    // std::fs::remove_dir_all("./hacash_mainnet_data");

    // main_test8327459283();
    // main_test_vecspeed387425983();
    // main_test28374659823746892();
    // return;
    // return main_test28374659823746892();
    
    let args: Vec<_> = std::env::args().collect();
    println!("{:?}", args);
    if args.len() >= 2 && args[1] == "--reptblk" {
        return main_test_report_test_block();
    }

    let inicnf = config::load_config();
    // deal datadir
    start_hacash_node(inicnf);

}



/**
 * create and start hash node
 */
fn start_hacash_node(iniobj: sys::IniObj) {

    println!("[Version] full node v{}, build time: {}, database type: {}.", 
        HACASH_NODE_VERSION, HACASH_NODE_BUILD_TIME, HACASH_STATE_DB_UPDT
    );

    use std::sync::mpsc::channel;
    let (cltx, clrx) = channel();
    ctrlc::set_handler(move || cltx.send(()).unwrap()); // ctrl+c to quit

    // println!("startHacashNode ini={:?}", iniobj);
    // mint
    crate::mint::action::init_reg();

    let mint_checker = Box::new(BlockMintChecker::new(&iniobj));

    // engine
    let dbv = HACASH_STATE_DB_UPDT;
    let engine = BlockEngine::open(&iniobj, dbv, mint_checker);
    let engptr: Arc<BlockEngine> = Arc::new(engine);

    // node
    let mut hnode = Arc::new(HacashNode::open(&iniobj, engptr.clone()));

    // server
    let server = DataServer::open(&iniobj, engptr.clone(), hnode.clone());
    std::thread::spawn(move||{
        server.start(); // http rpc 
    });

    // handle ctr+c to close
    let hn2 = hnode.clone();
    std::thread::spawn(move||{ loop{
        clrx.recv();
        hn2.close(); // ctrl+c to quit
    }});

    // start
    HacashNode::start(hnode);

    // on closed
    println!("\nHacash node closed.");
}










