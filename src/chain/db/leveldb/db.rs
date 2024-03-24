
pub struct LevelDB {
    // ldb: LevelDatabase<LDBKey>,
}


impl LevelDB {

    pub fn new(dir: &Path) -> LevelDB {
        // let mut opts = Options::new();
        // opts.create_if_missing = true;
        // let ldb = LevelDatabase::open(dir, opts).unwrap();
        LevelDB{
            // ldb: ldb,
        }
    }

    // get if find, bool is not check base
    pub fn get(&self, k: &[u8]) -> Option<Vec<u8>> {
        // self.ldb.get(ReadOptions::new(), k).unwrap()
        // not find return None
        None
    }

    // set
    pub fn set(&self, k: &[u8], v: &[u8]) {
        // self.ldb.put(WriteOptions::new(), k, k).unwrap();
    }

    // del
    pub fn del(&self, k: &[u8]) {
        // self.ldb.delete(WriteOptions::new(), k, k).unwrap();
    }
}
