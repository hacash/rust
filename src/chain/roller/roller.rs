

pub struct ChunkRoller {

    pub height: BlockHeight,
    pub hash: Hash,
    pub block: Box<dyn BlockPkg>,
    pub state: Arc<ChainState>,

    pub childs: RefCell<Vec<Arc<ChunkRoller>>>,
    pub parent: RefCell<Option<Weak<ChunkRoller>>>,

}


impl ChunkRoller {

    pub fn create(blkpkg: Box<dyn BlockPkg>, state: Arc<ChainState>) -> ChunkRoller {
        ChunkRoller{
            height: blkpkg.objc().height().clone(),
            hash: blkpkg.hash().clone(),
            block: blkpkg,
            state: state,
            childs: Vec::new().into(),
            parent: None.into(),
        }
    }

    pub fn push_child(&self, c: Arc<ChunkRoller>) {
        self.childs.borrow_mut().push(c);
    }

    pub fn set_parent(&mut self, p: Weak<ChunkRoller>) {
        *self.parent.borrow_mut() = Some(p);
    }

}

