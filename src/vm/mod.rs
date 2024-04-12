
use std::sync::{ Arc };

use crate::protocol::transaction::DynListVMAction;
use crate::sys::*;
use crate::interface::protocol::*;
use crate::interface::chain::*;
use crate::interface::vm::*;


mod stack;
mod memory;
mod storage;
mod bytecode;
mod ast;


pub mod action;
pub mod vm;


pub struct HacashVM {
    store: Arc<dyn Store>,
}




impl VM for HacashVM {

    fn new(ini: &IniObj, sto: Arc<dyn Store>) -> HacashVM {
        HacashVM{
            store: sto,
        }
    }

    fn exec(&self, env: &dyn ExecEnv, bst: &mut dyn State, con: &Vec<Box<dyn VMAction>>) -> RetErr {
        vm::do_exec(env, bst, self.store.as_ref(), con)
    }

}

