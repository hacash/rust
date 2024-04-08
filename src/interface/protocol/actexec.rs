
pub trait ExecResult {
    fn gas_use(&self) -> u32 { 0 }
    fn ret_val(&self) -> &[u8] { &[] }
    fn abort(&self) -> Option<Error>{ None }
}
/*
    fn burn_90(&self) -> bool { false } // is burning 90 persent fee
*/


pub trait ExecEnv<'a> {
    fn pending_height(&self) -> u64 { 0 }
    fn main_address(&self) -> &Address { panic_never_call_this!() }
    fn check_signature(&self, _: &Address) -> bool { false }
    fn address_list(&self) -> &[Address] { &[] }
    fn call_depth(&self) -> u32 { 0 }
}



pub trait ActExec {
    fn execute(&self, _: &dyn ExecEnv, _: &mut dyn State, _: &dyn Store) -> Box<dyn ExecResult> { panic_never_call_this!() }
}


