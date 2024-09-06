
// Hacash node
pub trait HNode: Send + Sync {

    fn submit_transaction(&self, _: &Box<dyn TxPkg>, is_async: bool) -> RetErr { panic_never_call_this!() }
    fn submit_block(&self, _: &Box<dyn BlockPkg>, is_async: bool) -> RetErr { panic_never_call_this!() }

    fn engine(&self) -> Arc<dyn Engine> { panic_never_call_this!() }
    fn tx_pool(&self) -> Arc<dyn TxPool> { panic_never_call_this!() }
}

