

pub struct BlockChainKernel {

    store: Rc<BlockStore>,
    state: RefCell<Weak<ChainState>>,

    sroot:  RefCell<Rc<ChunkRoller>>, // tree root block
    scusp: RefCell<Weak<ChunkRoller>>, // current latest block

    // insert lock
    isrlck: Mutex<bool>,
    updlck: RwLock<bool>,
}

