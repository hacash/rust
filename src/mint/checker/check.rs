


pub struct BlockMintChecker {

}


include!("consensus.rs");
include!("coinbase.rs");
include!("genesis.rs");

impl MintChecker for BlockMintChecker {

    fn consensus(&self, cbtx: &dyn Block) -> Option<Error> {
        impl_consensus(self, cbtx)
    }

    fn coinbase(&self, cbtx: &dyn Transaction) -> Option<Error> {
        impl_coinbase(self, cbtx)
    }

    fn genesis(&self, state: &mut dyn State) -> Option<Error> {
        impl_genesis(self, state)
    } 
}