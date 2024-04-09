
pub trait EngineRead {
    // key is height or hash
    fn block(&self, _: &dyn Serialize) -> Option<Box<dyn BlockPkg>> { panic_never_call_this!() }
    // key is hash
    fn tx(&self, _: &dyn Serialize) -> Option<Box<dyn TxPkg>> { panic_never_call_this!() }
    fn state(&self) -> Arc<dyn State> { panic_never_call_this!() }
    fn store(&self) -> Arc<dyn Store> { panic_never_call_this!() }

    // realtime average fee purity
    fn avgfee(&self) -> u32 { 0 }
}

pub trait Engine : EngineRead {
    // fn init(&self, _: &IniObj) -> Option<Error> { panic_never_call_this!() }
    // fn start(&self) -> Option<Error> { panic_never_call_this!() }
    fn insert(&self, _: Box<dyn BlockPkg>) -> RetErr { panic_never_call_this!() }
}


