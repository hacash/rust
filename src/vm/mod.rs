
use std::sync::{ Arc };

use crate::sys::*;
use crate::interface::protocol::*;
use crate::interface::chain::*;
use crate::interface::vm::*;



mod stack;
mod memory;
mod storage;
mod bytecode;
mod ast;
pub mod vm;



pub struct HacashVM {

}

impl VM for HacashVM {

    fn new() -> HacashVM {
        HacashVM{

        }
    }

    fn exec_tx(&self, h: u64, tx: &dyn TransactionRead, bst: &mut dyn State, sto: &dyn Store) -> Ret<(Box<dyn State>)> {
        let pedding_block_height = h;
    
        err!("")
    }
}
