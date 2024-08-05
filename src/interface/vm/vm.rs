

pub trait VM: Send + Sync {
    fn new(_: &IniObj, _: Arc<dyn Store>) -> Self where Self: Sized;
    fn exec(&self, _: &dyn ExecContext, _: &mut dyn State, _: &Vec<Box<dyn Action>>) -> RetErr { panic_never_call_this!() }
}


pub trait VMIvk {
    fn main_call(&mut self, entry: &Address, irs: &[u8]) -> Ret<Vec<u8>> { panic_never_call_this!() }
    fn sytm_call(&mut self, entry: &Address, fnty: u8, input: Vec<u8>) -> Ret<Vec<u8>> { panic_never_call_this!() }
}

