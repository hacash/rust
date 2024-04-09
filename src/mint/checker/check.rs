


pub struct BlockMintChecker {

}



impl MintChecker for BlockMintChecker {

    fn consensus(&self, cbtx: &dyn Block) -> RetErr {
        impl_consensus(self, cbtx)
    }

    fn coinbase(&self, cbtx: &dyn Transaction) -> RetErr {
        impl_coinbase(self, cbtx)
    }

    fn initialize(&self, state: &mut dyn StoreDB) -> RetErr {
        impl_initialize(self, state)
    } 

    fn genesis(&self) -> Box<dyn BlockPkg> {
        let gnsblk = create_genesis_block();
        // let body = gnsblk.serialize();
        let pkg = BlockPackage::new(Box::new(gnsblk));
        Box::new(pkg)
    }
}