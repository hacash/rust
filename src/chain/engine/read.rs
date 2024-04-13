
impl EngineRead for BlockEngine {

    fn state(&self) -> &dyn State { panic_never_call_this!() }

    fn store(&self) -> &dyn Store {
        self.store.as_ref()
    }

}
