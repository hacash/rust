


pub struct BlockMintChecker {

}



impl MintChecker for BlockMintChecker {

    fn consensus(&self, cbtx: &dyn Block) -> RetErr {
        impl_consensus(self, cbtx)
    }

    fn coinbase(&self, cbtx: &dyn Transaction) -> RetErr {
        impl_coinbase(self, cbtx)
    }

    fn genesis(&self, state: &mut dyn StoreDB) -> RetErr {
        impl_genesis(self, state)
    } 
}