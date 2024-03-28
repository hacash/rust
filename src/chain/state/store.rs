


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
    
    fn set(&self, p: &[u8], k: &dyn Serialize, v: &dyn Serialize) {
        let key = splice_key(p, k);
        let vdt = v.serialize();
        self.ldb.set(&key, &vdt); // disk
    }

    fn del(&self, p: &[u8], k: &dyn Serialize) {
        let key = splice_key(p, k);
        self.ldb.del(&key); // disk
    }

    

}


impl BlockStore {

}



