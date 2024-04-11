

pub trait MintChecker {
    // check
    fn consensus(&self, _: &dyn Block) -> RetErr;
    fn coinbase(&self, _: u64, _: &dyn Transaction) -> RetErr;
    // do
    fn initialize(&self, _: &mut dyn State) -> RetErr;
    // data
    fn genesis(&self) -> Box<dyn BlockPkg>;
    // actions
    fn actions(&self) -> Vec<Box<dyn Action>>;
}


