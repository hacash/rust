
impl EngineRead for BlockEngine {
    fn config(&self) -> &EngineConf {
        &self.cnf
    }

    fn state(&self) -> Arc<dyn State> {
        self.klctx.lock().unwrap().state.upgrade().unwrap()
    }

    fn store(&self) -> Arc<dyn Store> {
        self.store.clone()
    }

    fn latest_block(&self) -> Arc<dyn BlockPkg> {
        let curctx = self.klctx.lock().unwrap();
        let curchk = curctx.scusp.upgrade().unwrap();
        curchk.block.clone()
    }

    fn mint_checker(&self) -> Arc<dyn MintChecker> {
        self.mintk.clone().into()
    }

}
