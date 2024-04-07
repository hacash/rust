


/**
 * for Extend Action / HVM AST Node / Bytecode
 */
pub trait VMAction<'a> : Field {
    fn code(&self) -> u8 { 0 } // bytecode
    fn kind(&self) -> u16 { 0 } // action kind
    fn opnum(&self) -> u8 { 0 } // stack number of operate 
    fn childs(&'a self) -> &'a Vec<dyn VMAction> { panic_never_call_this!() }
    fn extbody(&'a self) -> &'a[u8] { &[] } // extend action body data

    // fn build(&mut self, _: &dyn ExtActCaller, _: &[u8]) -> Result<usize, Error> { panic_never_call_this!() }
}


/**
 * Extend Action
 */
pub trait Action : VMAction + Cutout + ActExec {
    fn burn_90(&self) -> bool { false } // is_burning_90_persent_fee
    fn req_sign(&self) -> HashSet<Address> { HashSet::new() } // request_need_sign_addresses
}



pub trait ActionContainer {
    fn build(&self, _: &[u8]) -> Ret<(Box<dyn Action>, usize)> { panic_never_call_this!() }
}



