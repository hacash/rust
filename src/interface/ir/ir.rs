

pub trait IRNode {
    fn bytecode(&self) -> u8;
    // fn childx(&self) -> &dyn IRNode { panic_never_call_this!() }
    // fn childy(&self) -> &dyn IRNode { panic_never_call_this!() }
    // fn childz(&self) -> &dyn IRNode { panic_never_call_this!() }
    // fn subnodes(&self) -> Vec<&dyn IRNode> { panic_never_call_this!() }
    // compile
    // fn parsing(&mut self, seek: &mut usize) -> RetErr { panic_never_call_this!() }
    fn codegen(&self) -> Vec<u8> { vec![self.bytecode()] }
}








