
pub struct BlockEngine {

    cnf: EngineConf,

    store: Arc<BlockStore>,

    klctx: Mutex<BlockRoller>, // change

    mintk: Box<dyn MintChecker>,
    // pub vmobj: Box<dyn VM>,
    // actns: Box<dyn >,

    // insert lock
    isrlck: Mutex<bool>,
}

impl BlockEngine {

    pub fn open(ini: &IniObj, mintk: Box<dyn MintChecker>) -> BlockEngine {
        let cnf = NewKernelConf(ini);
        // data dir
        std::fs::create_dir_all(&cnf.store_data_dir);
        std::fs::create_dir_all(&cnf.state_data_dir);
        std::fs::create_dir_all(&cnf.ctrkv_data_dir);
        // state & store
        let stoldb = BlockStore::open(&cnf.store_data_dir);
        let cstate = ChainState::open(&cnf.state_data_dir);
        let staptr = Arc::new(cstate);
        // base or genesis block
        let bsblk = load_base_block(mintk.as_ref(), &stoldb);
        let roller = BlockRoller::create(bsblk, staptr);
        // engine
        let engine = BlockEngine {
            cnf: cnf,
            store: Arc::new(stoldb),
            klctx: Mutex::new(roller),
            mintk: mintk,
            isrlck: Mutex::new(true),
        };
        engine
    }

    pub fn start(&mut self) -> RetErr {


        Ok(())
    }

    pub fn get_latest_state(&self) -> Option<Arc<dyn State>> {
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





