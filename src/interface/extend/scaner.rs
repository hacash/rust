

pub trait BlockScaner: Send + Sync {

    fn init(&mut self, _: &IniObj) -> RetErr { Ok(()) }
    fn exit(&self) -> RetErr { Ok(()) }

    fn start(&self) -> RetErr { Ok(()) } // handle loop
    fn serve(&self) -> RetErr { Ok(()) } // rpc server
    
    fn roll(&self, _: Arc<dyn BlockPkg>,  _: Arc<dyn State>, _: Arc<dyn Store>) -> RetErr { Ok(()) }

}