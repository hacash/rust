

pub trait StoreDB {
    fn read(&self, p: &[u8], k: &dyn Serialize) -> Option<Vec<u8>>;
    fn write(&self, p: &[u8], k: &dyn Serialize, v: &dyn Serialize);
    fn delete(&self, p: &[u8], k: &dyn Serialize);
}

pub trait StoreRead {

    fn latest(&self) -> Box<dyn StoreLatest>;

    fn block_bytes(&self, _: &Hash) -> Option<Vec<u8>>;
    fn block_ptr(&self, _: &BlockHeight) -> Option<Hash>;
    // load by height(5) or hash(32)
    fn block(&self, _: &dyn Serialize) -> Option<Box<dyn Block>>;

}

pub trait Store : StoreRead {

    fn init(&self) -> RetErr;

    fn save_block(&self, _: &dyn BlockPkg);
    fn set_block_point(&self, _: &BlockHeight, _: &Hash);

}


