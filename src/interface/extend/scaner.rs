

pub trait BlockScaner: Send + Sync {

    fn init(&mut self, _: &IniObj) -> RetErr { Ok(()) }
    fn start(&self) -> RetErr { Ok(()) }
    fn roll(&self, _: Arc<dyn BlockPkg>,  _: Arc<dyn State>, _: Arc<dyn Store>) -> RetErr { Ok(()) }

}