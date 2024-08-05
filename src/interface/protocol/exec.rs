
pub trait ExecResult {
    fn gasuse(&self) -> i64 { 0 }
    fn retval(&self) -> &[u8] { &[] }
    fn asval(mut self) -> Vec<u8> where Self: Sized { panic_never_call_this!() }
    fn abort(&self) -> &Option<Error>;
}
/*
    fn burn_90(&self) -> bool { false } // is burning 90 persent fee
*/


pub trait ExecContext {
    fn pending_height(&self) -> u64 { 0 }
    fn pending_hash(&self) -> &Hash { panic_never_call_this!() }
    fn main_address(&self) -> &Address { panic_never_call_this!() }
    fn addr_list(&self) -> &AddrOrList { panic_never_call_this!() }
    fn tx_fee(&self) -> &Amount { panic_never_call_this!() }
    fn check_signature(&mut self, adr: &Address) -> Ret<bool> { errf!("check {} signature error", adr.readable()) }
    // fn address_list(&self) -> &[Address] { &[] }
    fn call_depth(&self) -> u32 { 0 }
    fn fast_sync(&self) -> bool { false }
    fn actions(&self) -> &Vec<Box<dyn Action>> { panic_never_call_this!() }
    //
    fn vm(&mut self) -> Ret<&mut dyn VMIvk> { panic_never_call_this!() }
    fn syscall_check_true(&mut self, adr: &Address, f: u8, iptv: Vec<u8>) -> RetErr { panic_never_call_this!() }
    // fn vm_main_call(&mut self, entry: &Address, irs: &[u8]) -> Ret<Vec<u8>> { panic_never_call_this!() }
    // fn exec_act(&mut self) -> Box<dyn ExecResult> { panic_never_call_this!() }
}

pub trait ActExec {
    fn execute(&self, _: &mut dyn ExecContext, _: &mut dyn State, _: &dyn Store, _: i8) -> Ret<(i64, Vec<u8>)> { panic_never_call_this!() }
}

pub trait TxExec {
    fn execute(&self, hei: u64, _: &mut dyn State) -> RetErr { panic_never_call_this!() }
}


