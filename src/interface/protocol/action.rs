
pub trait Action : Field + Cutout + ActExec {
    fn kind(&self) -> u16;
    fn burn_90(&self) -> bool { false } // is_burning_90_persent_fee
    fn sign_req(&self) -> HashSet<Address> { HashSet::new() } // request_need_sign_addresses
}



pub trait ActionContainer {
    fn build(&self, _: &[u8]) -> Result<(Box<dyn Action>, usize), Error> { panic_never_call_this!() }
    fn invoke(&self) -> Option<Error> { panic_never_call_this!() }
}



