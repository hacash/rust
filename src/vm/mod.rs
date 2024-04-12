
use std::sync::{ Arc };

use crate::sys::*;
use crate::interface::protocol::*;
use crate::interface::chain::*;
use crate::interface::vm::*;



pub mod action;

mod stack;
mod memory;
mod storage;
mod bytecode;
mod ast;
pub mod vm;



pub struct HacashVM {
    store: Arc<dyn Store>,
}

impl VM for HacashVM {

    fn new(sto: Arc<dyn Store>) -> HacashVM {
        HacashVM{
            store: sto,
        }
    }

    fn exec_tx(&self, _: &dyn TransactionRead, bst: &mut dyn State) -> RetErr {
        err!("")
    }

}
