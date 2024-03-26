

pub trait MintChecker {
    // check
    fn consensus(&self, _: &dyn Block) -> RetErr;
    fn coinbase(&self, _: &dyn Transaction) -> RetErr;
    // do
    fn genesis(&self, _: &mut dyn State) -> RetErr;
}


