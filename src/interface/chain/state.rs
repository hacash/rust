

pub trait StateDB {
    fn get(&self, p: &[u8], k: &dyn Serialize) -> Option<Vec<u8>>;
    fn set(&self, p: &[u8], k: &dyn Serialize, v: &dyn Serialize);
    fn del(&self, p: &[u8], k: &dyn Serialize);

}

pub trait StateRead {

    // if not find return false
    fn load(&self, p: &[u8], k: &dyn Serialize, v: &mut dyn Parse) -> bool;

}

pub trait State {

    fn init(&self) {}

    fn save(&self, p: &[u8], k: &dyn Serialize, v: &dyn Serialize) { panic_never_call_this!() }
    fn flush(&self){ } // write data to disk and remove mem db

    // fn fork_next(&self) -> Box<dyn State>;
}



