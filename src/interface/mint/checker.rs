

pub trait MintChecker {
    // check
    fn consensus(&self, _: &dyn Block) -> Option<Error>;
    fn coinbase(&self, _: &dyn Transaction) -> Option<Error>;
    // do
    fn genesis(&self, _: &mut dyn State) -> Option<Error>;
}


