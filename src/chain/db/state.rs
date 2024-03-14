

enum DB {
    LevelDB,
    MemoryDB,
}


pub struct ChainState {

    db: DB,


}


impl StateDB for ChainState {
    
    fn get(&self, p: &str, k: &impl Serialize) -> Option<Vec<u8>> {
        let key = splice_key(p, k);

        None
    }
    
    fn set(&self, p: &str, k: &impl Serialize, v: &impl Serialize) {
        let key = splice_key(p, k);

    }

    fn del(&self, p: &str, k: &impl Serialize) {
        let key = splice_key(p, k);

    }

}