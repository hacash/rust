use std::*;
use std::path::*;

#[macro_use]
extern crate ini;


mod x16rs;

#[macro_use]
mod sys;
#[macro_use]
mod base;
mod config;
mod interface;
mod core;
mod protocol;
mod mint;
mod vm;
mod chain;

use crate::sys::*;
use crate::base::field::Hash;
use crate::interface::field::*;
use crate::core::account::Account;
use crate::chain::engine::*;

/**

sudo apt install g++
sudo apt install cmake

RUSTFLAGS="$RUSTFLAGS -Awarnings" RUST_BACKTRACE=1 cargo check / build / run

*/


fn main() {
    
    // main_test8327459283();

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
    let engine = BlockEngine::open(&iniobj);


}















fn main_test8327459283() {


    let hx = Hash::from_hex(b"faa1025aac192976049a91af8d552ebf5864f06732594b08456f6c7ab7d9a3e6");

    let srrstr: RetErr = errf!(" hash is {}", hx);
    println!("{}", srrstr.err().unwrap());



    let act1 = crate::protocol::action::ACTION_KIND_1;
    println!("{}", act1);

    let mut actobj = crate::protocol::action::HacTransfer::new();
    println!("{}", hex::encode(actobj.serialize()));




}





fn main_test134234() {


    let addrhac = core::field::AddrHac::new();

    println!("{} {} {}", "abc 123", s!("error"), addrhac.amount);

    let rshx = x16rs::x16rs_hash(1, &x16rs::calculate_hash(b"123456"));

    println!("{}", hex::encode(rshx));


    let pubkey = hex::decode("817ED5FC625752CBF027A39573E5F40FAC124AC1D983DD91C477C58F2A3BF983F4").unwrap();
    println!("{}", Account::to_readable(&Account::get_address_by_public_key(pubkey.try_into().unwrap())));

    

    // panic_never_call_this!();
}
