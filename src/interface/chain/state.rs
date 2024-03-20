

pub trait StateDB {
    fn get_at(&self, key: &[u8]) -> Option<Vec<u8>>;
    fn get(&self, p: &[u8], k: &dyn Serialize) -> Option<Vec<u8>>;
    fn set(&mut self, p: &[u8], k: &dyn Serialize, v: &dyn Serialize);
    fn del(&mut self, p: &[u8], k: &dyn Serialize);

}

pub trait StateRead : StateDB {

    // if not find return false
    fn load(&self, p: &[u8], k: &dyn Serialize, v: &mut dyn Parse) -> bool { panic_never_call_this!() }
}

pub trait State : StateRead {

    fn init(&self) {}

    fn save(&self, p: &[u8], k: &dyn Serialize, v: &dyn Serialize) { panic_never_call_this!() }
    fn flush(&self){ } // write data to disk and remove mem db

    // fn fork_next(&self) -> Box<dyn State>;
}



