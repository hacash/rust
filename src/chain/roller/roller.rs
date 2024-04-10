

pub struct StateRoller {
    
    pub state: Weak<ChainState>, // current latest state
    pub scusp: Weak<RollChunk>, // current latest block

    pub sroot: Arc<RollChunk>, // tree root block

}


impl StateRoller {

    pub fn create(blkpkg: Box<dyn BlockPkg>, state: Arc<ChainState>) -> StateRoller {
        let chunk = RollChunk::create(blkpkg, state.clone());
        let ckptr = Arc::new(chunk);
        StateRoller {
            state: Arc::downgrade(&state),
            scusp: Arc::downgrade(&ckptr),
            sroot: ckptr,
        }
    }


}