
pub trait ExecResult {
    fn gas_use(&self) -> u32 { 0 }
    fn ret_val<'a>(&'a self) -> &'a[u8] { &[] }
    fn abort(&self) -> Option<Error>{ None }
}
/*
    fn burn_90(&self) -> bool { false } // is burning 90 persent fee
*/


pub trait ExecEnv<'a> {
    fn main_address(&'a self) -> &'a Address { panic_never_call_this!() }
    fn check_signature(&self, _: &Address) -> bool { false }
    fn address_list(&'a self) -> &'a[Address] { &[] }
}



pub trait ActExec {
    fn execute(&self, _: &dyn ExecEnv, _: &mut dyn State, _: &dyn Store) -> Box<dyn ExecResult> { panic_never_call_this!() }
}


