

pub trait VM {
    fn new(sto: Arc<dyn Store>) -> Self where Self: Sized;
    fn reg_acts(&mut self, _: Vec<Box<dyn Action>>) { panic_never_call_this!() } // maybe panic
    fn exec_tx(&self, _: &dyn TransactionRead, _: &mut dyn State) -> Ret<(Box<dyn State>)> { panic_never_call_this!() }
    fn exec_block(&self, _: &dyn Block, _: &mut dyn State) -> Ret<(Box<dyn State>)> { panic_never_call_this!() }
}


