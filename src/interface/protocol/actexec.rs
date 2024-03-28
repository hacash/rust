
pub trait ExecResult {
    fn gas_use(&self) -> u32 { 0 }
    fn ret_val<'a>(&'a self) -> &'a[u8] { &[] }
    fn burn_90(&self) -> bool { false } // is burning 90 persent fee
    fn abort(&self) -> Option<Error>{ None }
}


pub trait ExecEnv {
    fn main_address(&self) -> Address { panic_never_call_this!() }
    fn check_signature(&self, _: &Address) -> bool { false }
}



pub trait ActExec {
    fn execute(&self, _: &dyn ExecEnv, _: &mut dyn State, _: &dyn Store) -> Box<dyn ExecResult> { panic_never_call_this!() }
}


