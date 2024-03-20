

pub struct ChainState {

    leveldb: RefCell<LevelDB>,
    delkeys: HashMap<Vec<u8>, ()>,

    baseptr: Option<Weak<ChainState>>,

}


impl StateDB for ChainState {

    fn get_at(&self, key: &[u8]) -> Option<Vec<u8>> {
        // check delete
        if let Some(_) = self.delkeys.get(key) {
            return None // delete
        }
        // check db, mem or disk 
        let v = self.leveldb.borrow_mut().get(&key);
        if let Some(v) = v {
            return Some(v) // find
        }
        // check base
        if let Some(b) = &self.baseptr {
            let b = b.upgrade().unwrap();
            return b.get_at(&key)
        }
        // none
        None
    }
    
    fn get(&self, p: &[u8], k: &dyn Serialize) -> Option<Vec<u8>> {
        let key = splice_key(p, k);
        self.get_at(&key)
    }
    
    fn set(&mut self, p: &[u8], k: &dyn Serialize, v: &dyn Serialize) {
        let key = splice_key(p, k);
        let vdt = v.serialize();
        self.leveldb.borrow_mut().put(&key, &vdt);
    }

    fn del(&mut self, p: &[u8], k: &dyn Serialize) {
        let key = splice_key(p, k);
        self.delkeys.insert(key, ());
    }

}


impl StateRead for ChainState {


}


impl State for ChainState {


}



// fork temp 
pub fn fork_temp_state(base: Arc<ChainState>) -> ChainState {
    ChainState{
        // memdb
        leveldb: LevelDB::open("temp", rusty_leveldb::in_memory()).unwrap().into(),
        delkeys: HashMap::new(),
        baseptr: Some(Arc::<ChainState>::downgrade(&base))
    }
}