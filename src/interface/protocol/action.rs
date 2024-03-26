
pub trait Action : Field + Cutout + ActExec {
    fn get_kind(&self) -> u16;
    // fn is_burning_90_persent_fee(&self) -> bool { false }
    // fn request_need_sign_addresses(&self) -> HashSet<Address> { HashSet::new() }
}



pub trait ActionContainer {
    fn build(&self, _: &[u8]) -> Result<(Box<dyn Action>, usize), Error> { panic_never_call_this!() }
    fn invoke(&self) -> Option<Error> { panic_never_call_this!() }
}



