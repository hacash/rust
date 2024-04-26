

#[derive(Clone)]
pub struct BlockMintChecker {

}

impl BlockMintChecker {
    pub fn create() -> BlockMintChecker {
        BlockMintChecker{}
    }
}


impl MintChecker for BlockMintChecker {

    fn consensus(&self, cbtx: &dyn Block) -> RetErr {
        impl_consensus(self, cbtx)
    }

    fn coinbase(&self, height: u64, cbtx: &dyn Transaction) -> RetErr {
        impl_coinbase(self, height, cbtx)
    }

    fn initialize(&self, state: &mut dyn State) -> RetErr {
        impl_initialize(self, state)
    } 

    fn genesis(&self) -> Arc<dyn BlockPkg> {
        genesis_block_ptr()
    }
    fn genesis_block(&self) -> Box<dyn BlockPkg> {
        genesis_block_pkg()
    }

    fn actions(&self) -> Vec<Box<dyn Action>> {
        vec![]
    }
}