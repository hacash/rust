
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

    // insert lock
    isrlck: Mutex<bool>,
}


impl BlockEngine {

    pub fn open(ini: &IniObj, dbv: u32, mintk: Box<dyn MintChecker>) -> BlockEngine {
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
            isrlck: Mutex::new(true),
        };
        // rebuild unstable blocks
        // if database upgrade, rebuild all block
        engine.rebuild_unstable_blocks(); // 
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

    pub fn try_execute_tx(&self, tx: &dyn Transaction) -> RetErr {
        let sta = self.get_latest_state();
        if let None = sta {
            return errf!("block engine not yet")
        }
        let mut sub_state = fork_sub_state(sta.unwrap());
        let height = self.get_latest_height().uint() + 1; // next height
        let blkhash = Hash::cons([0u8; 32]); // empty hash
        // exec
        exec_tx_actions(false, height, blkhash, &mut sub_state, self.store.as_ref(), tx.as_read())?;
        tx.execute(height, &mut sub_state)
    } 


}





