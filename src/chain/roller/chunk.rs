

pub struct RollChunk {

    pub height: BlockHeight,
    pub hash: Hash,
    pub block: Box<dyn BlockPkg>,
    pub state: Arc<ChainState>,

    pub childs: RefCell<Vec<Arc<RollChunk>>>,
    pub parent: RefCell<Weak<RollChunk>>,

}


impl RollChunk {

    pub fn create(blkpkg: Box<dyn BlockPkg>, state: Arc<ChainState>) -> RollChunk {
        RollChunk{
            height: blkpkg.objc().height().clone(),
            hash: blkpkg.hash().clone(),
            block: blkpkg,
            state: state,
            childs: Vec::new().into(),
            parent: Weak::new().into(), // none
        }
    }

    pub fn push_child(&self, c: Arc<RollChunk>) {
        self.childs.borrow_mut().push(c);
    }

    pub fn set_parent(&self, p: Arc<RollChunk>) {
        *self.parent.borrow_mut() = Arc::downgrade(&p).into();
    }
    pub fn drop_parent(&self) {
        *self.parent.borrow_mut() = Weak::new().into();
    }

}

