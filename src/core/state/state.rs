
pub struct ChainState {

    // db: DB,
    memk: MemoryDB,
    disk: Arc<LevelDB>,
    base: Weak<ChainState>,
}

impl ChainState {

    pub fn copy_ldb(&self) -> Arc<LevelDB> {
        self.disk.clone()
    }
    
}

// for debug
/* impl Drop for ChainState {
    fn drop(&mut self) {
        println!("ChainState drop !");
    }
}*/


impl StoreDB for ChainState {

    fn get_at(&self, key: &[u8]) -> Option<Bytes> {
        // check mem
        if let Some(dt) = self.memk.get(key) {
            // find the key
            if let MemdbItem::Delete = dt {
                return None // delete mark
            }else if let MemdbItem::Value(v) = dt {
                return Some(Bytes::Mem(v.clone())) // find
            }
        }
        // is have base db
        if let Some(basedb) = self.base.upgrade() {
            // read from base
            return basedb.get_at(key) 
        }
        // no base ptr, check disk db // resd disk final
        match self.disk.get_at(key) {
            Some(rb) => Some(Bytes::Raw(rb)),
            _ => None, // not find
        }
    }
    
    fn set_at(&mut self, k: &[u8], v: Vec<u8>) {
        self.memk.set(k, &v); // local mem
    }

    fn del_at(&mut self, k: &[u8]) {
        self.memk.del(&k); // local mem
    }

    fn get(&self, p: &[u8], k: &dyn Serialize) -> Option<Bytes> {
        let key = splice_key(p, k);
        self.get_at(&key)
    }
    
    fn set(&mut self, p: &[u8], k: &dyn Serialize, v: &dyn Serialize) {
        let key = splice_key(p, k);
        let vdt = v.serialize();
        self.set_at(&key, vdt);
    }

    fn del(&mut self, p: &[u8], k: &dyn Serialize) {
        let key = splice_key(p, k);
        self.del_at(&key); // local mem
    }

}

impl State for ChainState {

}


impl ChainState {

    pub fn open(dir: &Path) -> ChainState {
        let ldb = LevelDB::open(dir);
        ChainState{
            memk: MemoryDB::new(),
            disk: Arc::new(ldb),
            base: Weak::new().into(), // no base
        }
    }

    pub fn flush_disk(&self) {
        impl_flush_disk(self)
    }

}

