


pub struct BlockStore {
    ldb: Arc<LevelDB>,
}


impl StoreDB for BlockStore {

    fn get_at(&self, key: &[u8]) -> Option<Bytes> {
        // read disk
        match self.ldb.get_at(key) {
            Some(rb) => Some(Bytes::Raw(rb)),
            _ => None, // not find
        }
    }
    
    fn get(&self, p: &[u8], k: &dyn Serialize) -> Option<Bytes> {
        let key = splice_key(p, k);
        self.get_at(&key)
    }
    
    fn put(&self, p: &[u8], k: &dyn Serialize, v: &dyn Serialize) {
        let key = splice_key(p, k);
        let vdt = v.serialize();
        self.ldb.put(&key, &vdt); // disk
    }

    fn rm(&self, p: &[u8], k: &dyn Serialize) {
        let key = splice_key(p, k);
        self.ldb.rm(&key); // disk
    }



}

impl Store for BlockStore {
}


impl BlockStore {

    pub fn open(dir: &Path) -> BlockStore {
        let ldb = LevelDB::open(dir);
        BlockStore{
            ldb: Arc::new(ldb),
        }
    }


}






