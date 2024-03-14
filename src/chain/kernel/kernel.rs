

pub struct BlockChainKernel {

    store: Rc<BlockStore>,
    state: Weak<ChainState>,

    brick: Rc<StateBrick>,

}

