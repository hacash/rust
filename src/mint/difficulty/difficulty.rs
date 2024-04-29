
#[derive(Clone)]
pub struct DifficultyGnr {
    cnf: MintConf,
    block_times: Arc<Mutex<HashMap<u64,u64>>>, // height => time sec 
}

impl DifficultyGnr {

    pub fn new(cnf: MintConf) -> DifficultyGnr {
        DifficultyGnr {
            cnf: cnf,
            block_times: Arc::default(),
        }
    }

}





impl DifficultyGnr {

    pub fn req_cycle_time(&self, hei: u64, sto: &dyn Store) -> u64 {
        let cylnum = self.cnf.difficulty_adjust_blocks;
        if hei < cylnum { // 288
            return genesis_block().timestamp().uint()
        }
        let cylhei = hei / cylnum * cylnum;
        let mut cache = self.block_times.lock().unwrap();
        if let Some(blk_time) = cache.get(&cylhei) {
            return *blk_time // find in cache
        }
        // read from database
        let store = CoreStoreDisk::wrap(sto);
        let blkhx = store.blockptr(&BlockHeight::from(cylhei)).unwrap();
        let blkdts = store.blockdata(&blkhx).unwrap();
        let mut intro = BlockIntro::new();
        intro.parse(blkdts.as_ref(), 0).unwrap();
        // get time
        let cyltime = intro.head.timestamp.uint();
        cache.insert(cylhei, cyltime);
        if cache.len() as u64 > cylnum {
            cache.clear(); // clear
        }
        // ok
        cyltime
    }

}