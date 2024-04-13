

pub struct BlockRoller {
    
    pub state: Weak<ChainState>, // current latest state
    pub scusp: Weak<RollChunk>, // current latest block

    pub sroot: Arc<RollChunk>, // tree root block

}


impl BlockRoller {

    pub fn create(blkpkg: Box<dyn BlockPkg>, state: Arc<ChainState>) -> BlockRoller {
        let chunk = RollChunk::create(blkpkg, state.clone());
        let ckptr = Arc::new(chunk);
        BlockRoller {
            state: Arc::downgrade(&state),
            scusp: Arc::downgrade(&ckptr),
            sroot: ckptr,
        }
    }

    pub fn root_height(&self) -> BlockHeight {
        self.sroot.height.clone()
    }

    pub fn last_height(&self) -> BlockHeight {
        self.scusp.upgrade().unwrap().height.clone()
    }


}