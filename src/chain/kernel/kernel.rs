

pub struct BlockChainKernel {

    store: Rc<BlockStore>,
    state: Weak<ChainState>,

    sroot: Rc<ChunkRoller>, // tree root block
    scusp: Weak<ChunkRoller>, // current latest block

    


}

