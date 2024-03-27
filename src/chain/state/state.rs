
pub struct ChainState {

    // db: DB,
    memk: MemoryDB,
    disk: Arc<LevelDB>,
    base: RwLock<Option<Weak<ChainState>>>,
}

impl ChainState {

    pub fn copy_ldb(&self) -> Arc<LevelDB> {
        self.disk.clone()
    }
    
}


impl StateDB for ChainState {

    fn get_at(&self, key: &[u8]) -> Option<Vec<u8>> {
        // is have base db
        let basedb = self.base.read().unwrap();
        let basedb = basedb.as_ref();
        if let None = basedb {
            // no base ptr, check disk db
            return self.disk.get(key) // search disk final
        }
        // first, check local mem
        if let Some(dt) = self.memk.get(key) {
            // find the key
            if let MemdbItem::Delete = dt {
                return None // delete mark
            }else if let MemdbItem::Value(v) = dt {
                return Some(v.clone()) // find
            }
        }
        // must have base ptr, check base
        basedb.unwrap().upgrade().unwrap().get_at(key) // search from base ptr
    }
    
    fn get(&self, p: &[u8], k: &dyn Serialize) -> Option<Vec<u8>> {
        let key = splice_key(p, k);
        self.get_at(&key)
    }
    
    fn set(&mut self, p: &[u8], k: &dyn Serialize, v: &dyn Serialize) {
        let key = splice_key(p, k);
        let vdt = v.serialize();
        self.memk.set(&key, &vdt); // local mem
    }

    fn del(&mut self, p: &[u8], k: &dyn Serialize) {
        let key = splice_key(p, k);
        self.memk.del(&key); // local mem
    }

}


impl StateRead for ChainState {


}


impl State for ChainState {

    fn flush_disk(&self) {
        impl_flush_disk(self)
    }

}

