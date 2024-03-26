

struct KernelCtx {
    
    state: Weak<ChainState>,

    sroot: Arc<ChunkRoller>, // tree root block
    scusp: Weak<ChunkRoller>, // current latest block

}

pub struct BlockChainKernel {

    cnf: KernelConf,

    store: Arc<BlockStore>,

    klctx: RwLock<RefCell<KernelCtx>>,

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

    pub fn get_latest_state(&self) -> Arc<dyn State> {
        let ctx = self.klctx.read().unwrap();
        let ctx = ctx.borrow();
        if let Some(st) = ctx.state.upgrade() {
            return st
        }
        if let Some(sc) = ctx.scusp.upgrade() {
            return sc.state.clone()
        }
        // base
        ctx.sroot.state.clone()
    }
}





