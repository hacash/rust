
pub struct ChainState {

    db: DB,
    base: Weak<ChainState>,

}

impl ChainState {

    // pub fn drop_parent(&self) {
    //     self.base = None;
    // }
    
}


impl StateDB for ChainState {

    fn get_at(&self, key: &[u8]) -> Option<Vec<u8>> {
        // check delete
        let (res, notbase) = self.db.get(key);
        if let Some(v) = res {
            return Some(v) // ok find
        }
        if notbase {
            return res // find or not find
        }
        // check base
        if let Some(b) = self.base.upgrade() {
            return b.get_at(key)
        }
        // final not find
        None
    }
    
    fn get(&self, p: &[u8], k: &dyn Serialize) -> Option<Vec<u8>> {
        let key = splice_key(p, k);
        self.get_at(&key)
    }
    
    fn set(&mut self, p: &[u8], k: &dyn Serialize, v: &dyn Serialize) {
        let key = splice_key(p, k);
        let vdt = v.serialize();
        self.db.set(&key, &vdt);
    }

    fn del(&mut self, p: &[u8], k: &dyn Serialize) {
        let key = splice_key(p, k);
        self.db.del(&key);
    }

}


impl StateRead for ChainState {


}


impl State for ChainState {
    // fn fork_sub(&mut self) -> Box<dyn State> {
    //     impl_fork_sub(&self)
    // }
}

