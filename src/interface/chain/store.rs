

pub trait StoreDB: Send + Sync {
    fn get_at(&self, key: &[u8]) -> Option<Bytes>;
    fn get(&self,     p: &[u8], k: &dyn Serialize) -> Option<Bytes>;
    fn load(&self,    p: &[u8], k: &dyn Serialize, v: &mut dyn Parse) -> bool { 
        match self.get(p, k) {
            None => false,
            Some(dt) => {
                v.parse(&dt, 0).unwrap(); // maybe panic
                true
            }
        }
    }
    // mem
    
    fn set_at(&mut self, k: &[u8], v: Vec<u8>) { panic_never_call_this!() }
    fn del_at(&mut self, k: &[u8]) { panic_never_call_this!() }
    fn set(&mut self, p: &[u8], k: &dyn Serialize, v: &dyn Serialize) { panic_never_call_this!() }
    fn del(&mut self, p: &[u8], k: &dyn Serialize) { panic_never_call_this!() }
    // disk
    fn put(&self, p: &[u8], k: &dyn Serialize, v: &dyn Serialize) { panic_never_call_this!() }
    fn rm( &self, p: &[u8], k: &dyn Serialize) { panic_never_call_this!() }
}


pub trait Store : StoreDB + Send {
}









/* 
pub trait StoreDB {
    fn read(&self, p: &[u8], k: &dyn Serialize) -> Option<Vec<u8>>;
    fn write(&self, p: &[u8], k: &dyn Serialize, v: &dyn Serialize);
    fn delete(&self, p: &[u8], k: &dyn Serialize);
}

pub trait StoreRead {

    fn latest(&self) -> Box<dyn StoreLatest>;

    fn block_bytes(&self, _: &Hash) -> Option<Vec<u8>>;
    fn block_ptr(&self, _: &BlockHeight) -> Option<Hash>;
    // load by height(5) or hash(32)
    fn block(&self, _: &dyn Serialize) -> Option<Box<dyn Block>>;

}

pub trait Store : StoreRead {

    fn init(&self) -> RetErr;

    fn save_block(&self, _: &dyn BlockPkg);
    fn set_block_point(&self, _: &BlockHeight, _: &Hash);

}

*/
