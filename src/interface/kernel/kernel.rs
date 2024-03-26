
pub trait KernelRead {
    fn block(&self, _: &dyn Serialize) -> Option<Box<dyn BlockPkg>> { panic_never_call_this!() }
    fn store(&self) { panic_never_call_this!() }
    // realtime average fee purity
    fn avgfee(&self) -> u32 { 0 }
}

pub trait Kernel : KernelRead {
    // fn init(&self, _: &IniObj) -> Option<Error> { panic_never_call_this!() }
    // fn start(&self) -> Option<Error> { panic_never_call_this!() }
    fn insert(&mut self, _: Box<dyn BlockPkg>) -> Option<Error> { panic_never_call_this!() }
}


