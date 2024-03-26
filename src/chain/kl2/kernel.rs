

pub struct BlockChainKernel {

    cnf: KernelConf,

    store: BlockStore,
    state: Weak<ChainState>,

    sroot: Rc<ChunkRoller>, // tree root block
    scusp: Weak<ChunkRoller>, // current latest block

    mintk: Box<dyn MintChecker>,

    // insert lock
    isrlck: Mutex<bool>,
    // updlck: RwLock<bool>,
}
