

pub trait StoreDB {
    fn read(&self, p: &str, k: &impl Serialize) -> Option<Vec<u8>>;
    fn write(&self, p: &str, k: &impl Serialize, v: &impl Serialize);
    fn delete(&self, p: &str, k: &impl Serialize);
}

pub trait StoreRead {

    fn latest(&self) -> Box<dyn StoreLatest>;

    fn block_bytes(&self, _: &Hash) -> Option<Vec<u8>>;
    fn block_ptr(&self, _: BlockHeight) -> Option<Hash>;
    // load by height(5) or hash(32)
    fn block(&self, _: &impl Serialize) -> Option<Box<dyn Block>>;

}

pub trait Store : StoreRead {

    fn init(&self) -> Option<Error>;

    fn save_block(&self, _: &impl BlockPkg);
    fn set_block_cursor(&self, _: BlockHeight, _: Hash);

}


