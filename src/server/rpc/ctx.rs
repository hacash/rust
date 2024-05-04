
/********************/

pub type ChainEngine = Arc<dyn Engine>;
pub type ChainNode = Arc<dyn HNode>;
pub type BlockCaches = Arc<Mutex<VecDeque<Arc<dyn BlockPkg>>>>;

#[derive(Clone)]
pub struct ApiCtx {
    pub engine: ChainEngine,
    pub hcshnd: ChainNode,
    pub blocks: BlockCaches,
    blocks_max: usize, // 4
}

impl ApiCtx {
    pub fn new(eng: ChainEngine, nd: ChainNode) -> ApiCtx {
        ApiCtx{
            engine: eng,
            hcshnd: nd,
            blocks: Arc::default(),
            blocks_max: 4,
        }
    }

    // load block from cache or disk, key = height or hash
    pub fn load_block(&self, store: &CoreStoreDisk, key: &String) -> Ret<Arc<dyn BlockPkg>> {
        let mut hash = Hash::cons([0u8; 32]);
        let mut height = BlockHeight::from(0);
        if key.len() == 64 {
            if let Ok(hx) = hex::decode(key) {
                hash = Hash::cons(hx.try_into().unwrap());
            }
        }else{
            if let Ok(num) = key.parse::<u64>() {
                height = BlockHeight::from(num);
            }
        }
        // check cache
        let mut list = self.blocks.lock().unwrap();
        for blk in list.iter() {
            if height == blk.objc().height().uint() || hash == *blk.hash() {
                return Ok(blk.clone())
            }
        }
        // read from disk
        let mut blkdts;
        if height.uint() > 0 {
            blkdts = store.blockdatabyptr(&height);
        }else{
            blkdts = store.blockdata(&hash);
        }
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


