
pub trait EngineRead: Send + Sync {
    // key is height or hash
    // fn block(&self, _: &dyn Serialize) -> Option<Box<dyn BlockPkg>> { panic_never_call_this!() }
    // key is hash
    // fn tx(&self, _: &dyn Serialize) -> Option<Box<dyn TxPkg>> { panic_never_call_this!() }
    fn config(&self) -> &EngineConf { panic_never_call_this!() }

    fn state(&self) -> Arc<dyn State> { panic_never_call_this!() }
    fn store(&self) -> Arc<dyn Store> { panic_never_call_this!() }

    fn latest_block(&self) -> Arc<dyn BlockPkg> { panic_never_call_this!() }
    fn mint_checker(&self) -> Arc<dyn MintChecker> { panic_never_call_this!() }

    // realtime average fee purity
    // fn avgfee(&self) -> u32 { 0 }
}

pub trait Engine : EngineRead + Send + Sync {
    // fn init(&self, _: &IniObj) -> Option<Error> { panic_never_call_this!() }
    // fn start(&self) -> Option<Error> { panic_never_call_this!() }
    fn insert(&self, _: Box<dyn BlockPkg>) -> RetErr { panic_never_call_this!() }
    fn insert_sync(&self, _: u64, _: Vec<u8>) -> RetErr { panic_never_call_this!() }
}


