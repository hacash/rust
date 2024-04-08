

pub trait MintChecker {
    // check
    fn consensus(&self, _: &dyn Block) -> RetErr;
    fn coinbase(&self, _: &dyn Transaction) -> RetErr;
    // do
    fn initialize(&self, _: &mut dyn StoreDB) -> RetErr;
    // data
    fn genesis(&self) -> Box<dyn BlockPkg>;
}


