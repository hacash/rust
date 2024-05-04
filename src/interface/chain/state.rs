

pub trait State : StoreDB + Send {

    
    // fn fork_sub(&self) -> Box<dyn State>;
    // fn load(&self, data: &Bytes, v: &mut dyn Parse) -> RetErr { panic_never_call_this!() }

    // if not find return false
    // fn save(&mut self, p: &[u8], k: &dyn Serialize, v: &dyn Serialize) { panic_never_call_this!() }
    // fn drop(&mut self, p: &[u8], k: &dyn Serialize) { panic_never_call_this!() }

    // if v.len() == 0 , delete the key, is the db is disk will get panic!
    // fn memvks<'a>(&'a self) -> Vec<(&'a [u8], &'a [u8])> { panic_never_call_this!() } 


}



/* 
pub trait State : StateRead {

    // fn init(&self) {}

    // fn flush_disk(&self) { panic_never_call_this!() }


    // fn fork_sub(&self) -> Box<dyn State> { panic_never_call_this!() }
    // fn link(&mut self, _: Arc<dyn State>) { panic_never_call_this!() }
    // merge memdb data
    fn merge_copy(&mut self, src: &dyn State) -> RetErr { panic_never_call_this!() }
    // if the db is disk, merge will write/flush data to disk, or panic
    fn flush_disk(&self) { panic_never_call_this!() }
}
*/


