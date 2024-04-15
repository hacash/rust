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

use crate::tests::*;

/**

sudo apt install g++
sudo apt install cmake     

RUSTFLAGS="$RUSTFLAGS -Awarnings" RUST_BACKTRACE=1 cargo check / build / run

*/


fn main() {
    
    // delete datadir
    std::fs::remove_dir_all("./hacash_mainnet_data");

    // main_test8327459283();
    // main_test_vecspeed387425983();

    let inicnf = read_config();
    start_hacash_node(inicnf);

}

// load config
fn read_config() -> sys::IniObj {

    let args: Vec<String> = env::args().collect();
    let exedir = env::current_exe().unwrap();
    let mut inicnfpath = exedir.parent().unwrap().to_path_buf();
    let mut inifp = "./hacash.config.ini".to_string();
    if args.len() >= 2 {
        inifp = args[1].clone();
    }
    if inifp.starts_with("./") {
        // inifp = inifp[2..].to_string();
    }
    inicnfpath = inicnfpath.join(PathBuf::from(inifp));
    // println!("{:?} {:?}", args, exedir);
    // println!("{:?}", inicnfpath.to_str().unwrap());
    // read
    if inicnfpath.exists() {
        ini!(inicnfpath.to_str().unwrap())
    }else{
        sys::IniObj::new()
    }
}

// start node
fn start_hacash_node(iniobj: sys::IniObj) {
    // println!("startHacashNode ini={:?}", iniobj);
    let mint_checker = Box::new(BlockMintChecker::create());
    let engine = BlockEngine::open(&iniobj, mint_checker);
    let engptr = Arc::new(engine);
    // test
    engine_test_3(engptr);

}










