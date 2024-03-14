

enum DB {
    Disk(LevelDB),
    Memory(MemoryDB),
}


pub struct ChainState {

    db: DB,

    base: Option<Weak<ChainState>>,
    subs: Vec<Rc<ChainState>>,

}


impl StateDB for ChainState {
    
    fn get(&self, p: u8, k: &impl Serialize) -> Option<Vec<u8>> {
        let key = splice_key(p, k);

        None
    }
    
    fn set(&self, p: u8, k: &impl Serialize, v: &impl Serialize) {
        let key = splice_key(p, k);

    }

    fn del(&self, p: u8, k: &impl Serialize) {
        let key = splice_key(p, k);

    }

}