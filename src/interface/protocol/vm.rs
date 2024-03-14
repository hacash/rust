
/**
 * for HVM AST Node or Bytecode
 */
pub trait VMAction {
    fn get_code(&self) -> u8 { 0 } // bytecode
    fn operands(&self) -> u8 { 0 } // stack number of operate 
    
}

