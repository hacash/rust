

pub struct ChunkRoller {

    pub height: BlockHeight,
    pub hash: Hash,
    pub block: BlockPkgInst,
    pub state: Arc<ChainState>,

    pub childs: Vec<Arc<ChunkRoller>>,
    pub parent: Option<Weak<ChunkRoller>>,

}


