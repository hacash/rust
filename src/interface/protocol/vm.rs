
/**
 * for HVM AST Node or Bytecode
 */
pub trait VMAction : Field {
    fn get_code(&self) -> u8 { 0 } // bytecode
    fn operands(&self) -> u8 { 0 } // stack number of operate 
    
    fn ext_body(&self) -> &[u8] { &[] }

    // test
    fn run_test(&self, _: &dyn Serialize) {}
}





