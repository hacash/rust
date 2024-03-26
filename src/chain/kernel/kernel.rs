

struct StateRoller {
    
    state: Weak<ChainState>,

    sroot: Arc<RollChunk>, // tree root block
    scusp: Weak<RollChunk>, // current latest block

}

pub struct BlockChainKernel {

    cnf: KernelConf,

    store: Arc<BlockStore>,

    klctx: RwLock<StateRoller>, // change

    mintk: Box<dyn MintChecker>,

    // insert lock
    isrlck: Mutex<bool>,
    // updlck: RwLock<bool>,
}

impl BlockChainKernel {
    
    pub fn init(&mut self, ini: &IniObj) -> Option<Error> {
        let cnf = NewKernelConf(ini);
        // create data dir
        std::fs::create_dir_all(&cnf.store_data_dir);
        std::fs::create_dir_all(&cnf.state_data_dir);
        // ok
        self.cnf = cnf;
        None
    }

    pub fn start(&mut self) -> Option<Error> {


        None
    }

    pub fn get_latest_state(&self) -> Option<Arc<dyn State>> {
        let ctx = self.klctx.try_read();
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





