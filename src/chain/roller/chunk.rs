

pub struct RollChunk {

    pub height: BlockHeight,
    pub hash: Hash,
    pub block: Box<dyn BlockPkg>,
    pub state: Arc<ChainState>,

    pub childs: RefCell<Vec<Arc<RollChunk>>>,
    pub parent: Weak<RollChunk>,

}


impl RollChunk {

    pub fn create(blkpkg: Box<dyn BlockPkg>, state: Arc<ChainState>) -> RollChunk {
        RollChunk{
            height: blkpkg.objc().height().clone(),
            hash: blkpkg.hash().clone(),
            block: blkpkg,
            state: state,
            childs: Vec::new().into(),
            parent: Weak::new(), // none
        }
    }

    pub fn push_child(&self, c: Arc<RollChunk>) {
        self.childs.borrow_mut().push(c);
    }

    pub fn set_parent(&mut self, p: Arc<RollChunk>) {
        self.parent = Arc::downgrade(&p).into();
    }

    pub fn print(&self) -> String {
        let mark = &self.hash.as_ref()[30..];
        format!("{}:{}", self.height.to_u64(), hex::encode(mark))
    }

}

