
pub trait ExecResult {
    fn gasuse(&self) -> u32 { 0 }
    fn retval(&self) -> &[u8] { &[] }
    fn abort(&self) -> &Option<Error>;
}
/*
    fn burn_90(&self) -> bool { false } // is burning 90 persent fee
*/


pub trait ExecEnv {
    fn pending_height(&self) -> u64 { 0 }
    fn pending_hash(&self) -> &Hash { panic_never_call_this!() }
    fn main_address(&self) -> &Address { panic_never_call_this!() }
    fn tx_fee(&self) -> &Amount { panic_never_call_this!() }
    fn check_signature(&self, adr: &Address) -> RetErr { errf!("check {} signature error", adr.to_readable()) }
    // fn address_list(&self) -> &[Address] { &[] }
    fn call_depth(&self) -> u32 { 0 }
}

pub trait ActExec {
    fn execute(&self, _: &dyn ExecEnv, _: &mut dyn State, _: &dyn Store) -> Box<dyn ExecResult> { panic_never_call_this!() }
}

pub trait TxExec {
    fn execute(&self, hei: u64, _: &mut dyn State) -> RetErr { panic_never_call_this!() }
}


