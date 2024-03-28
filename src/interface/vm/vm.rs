

pub trait VM {
    fn new() -> Self where Self: Sized;
    fn exec_tx(&self, h: u64, tx: &dyn TransactionRead, bst: &mut dyn State, sto: &dyn Store) -> Ret<(Box<dyn State>)>;
}




/**
 * for HVM AST Node or Bytecode
 */
pub trait VMAction : Field {
    fn get_code(&self) -> u8 { 0 } // bytecode
    fn operands(&self) -> u8 { 0 } // stack number of operate 
    
    fn ext_body(&self) -> &[u8] { &[] }

    fn build(&mut self, _: &dyn ExtActCaller, _: &[u8]) -> Result<usize, Error> { panic_never_call_this!() }

    // test
    fn run_test(&self, _: &dyn Serialize) {}
}





