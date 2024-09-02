
#[derive(Debug)]
struct RollerChangeStatus {
    roll: bool,   
    append: bool, 
    switchfork: bool,
}

impl RollerChangeStatus {
    fn new() -> RollerChangeStatus {
        RollerChangeStatus{
            roll: false,   
            append: false, 
            switchfork: false,
        }
    }
}


/**********************/

pub struct BlockEngine {

    cnf: EngineConf,

    store: Arc<BlockStore>,

    klctx: Mutex<BlockRoller>, // change

    mintk: Box<dyn MintChecker>,
    // actns: Box<dyn >,
    blkscaner: Arc<dyn BlockScaner>,

    // insert lock
    isrlck: Mutex<bool>,

    rctblks: Mutex<VecDeque<Arc<RecentBlockInfo>>>,
    avgfees: Mutex<VecDeque<u64>>,
    
}


impl BlockEngine {

    pub fn open(ini: &IniObj, dbv: u32, mintk: Box<dyn MintChecker>, blkscaner: Arc<dyn BlockScaner>) -> BlockEngine {
        let cnf = EngineConf::new(ini, dbv);
        // load store
        std::fs::create_dir_all(&cnf.store_data_dir);
        let stoldb = BlockStore::open(&cnf.store_data_dir);
        // if database upgrade
        let is_database_upgrade = false == cnf.state_data_dir.exists();
        // start state
        std::fs::create_dir_all(&cnf.state_data_dir);
        let cstate = ChainState::open(&cnf.state_data_dir);
        let staptr = Arc::new(cstate);
        // base or genesis block
        let bsblk = match is_database_upgrade {
            true => mintk.genesis_block().into(), // rebuild all block
            false => load_base_block(mintk.as_ref(), &stoldb)
        };            
        let roller = BlockRoller::create(bsblk, staptr);
        let stoptr = Arc::new(stoldb);
        // engine
        let mut engine = BlockEngine {
            cnf: cnf,
            store: stoptr.clone(),
            klctx: Mutex::new(roller),
            mintk: mintk,
            blkscaner: blkscaner,
            isrlck: Mutex::new(true),
            rctblks: Mutex::default(),
            avgfees: Mutex::default(),
        };
        // rebuild unstable blocks
        // if database upgrade, rebuild all block
        engine.rebuild_unstable_blocks();
        // ok finish
        engine
    }


    pub fn get_latest_height(&self) -> BlockHeight {
        self.klctx.lock().unwrap().last_height()
    }


    pub fn get_latest_state(&self) -> Option<Arc<ChainState>> {
        let ctx = self.klctx.try_lock();
        if let Err(_) = ctx {
            return None // state busy !!!
        }
        let ctx = ctx.unwrap();
        if let Some(st) = ctx.state.upgrade() {
            return Some(st)
        }
        if let Some(sc) = ctx.scusp.upgrade() {
            return Some(sc.state.clone())
        }
        // base
        Some(ctx.sroot.state.clone())
    }


}





