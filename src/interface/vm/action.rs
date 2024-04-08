


/*
for HVM AST Node or Bytecode

pub trait VMAction : Field {
    fn code(&self) -> u8 { 0 } // bytecode
    fn operands(&self) -> u8 { 0 } // stack number of operate 
    fn body<'a>(&'a self) -> &'a[u8] { &[] } // extend body data

    fn build(&mut self, _: &dyn ExtActCaller, _: &[u8]) -> Result<usize, Error> { panic_never_call_this!() }
}

*/



