

pub struct ChunkRoller {

    height: BlockHeight, 
    block: BlockPkg,
    state: Rc<ChainState>,

    childs: Vec<Rc<ChunkRoller>>,
    parent: Option<Weak<ChunkRoller>>,

}


