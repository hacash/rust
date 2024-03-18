

enum DB {
    Disk(LevelDB),
    Memory(MemoryDB),
}


pub struct ChainState {

    db: DB,

    base: Option<Weak<ChainState>>,
    // subs: Vec<Arc<ChainState>>,

}


impl StateDB for ChainState {
    
    fn get(&self, p: &[u8], k: &dyn Serialize) -> Option<Vec<u8>> {
        let key = splice_key(p, k);

        None
    }
    
    fn set(&self, p: &[u8], k: &dyn Serialize, v: &dyn Serialize) {
        let key = splice_key(p, k);

    }

    fn del(&self, p: &[u8], k: &dyn Serialize) {
        let key = splice_key(p, k);

    }

}


impl State for ChainState {

}