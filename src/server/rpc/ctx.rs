
/********************/

pub type ChainEngine = Arc<dyn Engine>;
pub type BlockCaches = Arc<Mutex<VecDeque<Arc<dyn BlockPkg>>>>;

#[derive(Clone)]
pub struct ApiCtx {
    pub engine: ChainEngine,
    pub blocks: BlockCaches,
    blocks_max: usize, // 4
}

impl ApiCtx {
    pub fn new(eng: ChainEngine) -> ApiCtx {
        ApiCtx{
            engine: eng,
            blocks: Arc::default(),
            blocks_max: 4,
        }
    }

    // load block from cache or disk
    pub fn load_block(&self, store: &CoreStoreDisk, height: u64) -> Ret<Arc<dyn BlockPkg>> {
        // check cache
        let mut list = self.blocks.lock().unwrap();
        for blk in list.iter() {
            if height == blk.objc().height().uint() {
                return Ok(blk.clone())
            }
        }
        // read from disk
        let blkdts = store.blockdatabyptr(&BlockHeight::from(height));
        if let None = blkdts {
            return errf!("block not find")
        }
        let blkpkg = block::create_pkg(blkdts.unwrap());
        if let Err(e) = blkpkg {
            return errf!("block parse error: {}", &e)
        }
        let blkpkg = blkpkg.unwrap();
        // ok
        let blkcp: Arc<dyn BlockPkg> = blkpkg.into();
        list.push_front(blkcp.clone());
        if list.len() > self.blocks_max {
            list.pop_back(); // cache limit 
        }
        return Ok(blkcp)
    }
}


