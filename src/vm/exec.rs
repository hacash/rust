use std::sync::{ Arc };

use crate::sys::*;
use crate::interface::protocol::*;
use crate::interface::chain::*;



pub fn call_vm_exec_tx(h: u64, tx: &dyn TransactionRead, bs: &mut dyn State) -> Result<(Box<dyn State>), Error> {
    let pedding_block_height = h;

    




    err!("")
}