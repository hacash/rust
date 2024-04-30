

#[derive(Clone)]
pub struct BlockMintChecker {
    cnf: MintConf,
    difficulty: DifficultyGnr,
}

impl BlockMintChecker {
    pub fn new(ini: &IniObj) -> BlockMintChecker {
        let cnf = NewMintConf(ini);
        BlockMintChecker{
            cnf: cnf.clone(),
            difficulty: DifficultyGnr::new(cnf),
        }
    }
}


impl MintChecker for BlockMintChecker {
    fn config(&self) -> &MintConf {
        &self.cnf
    }

    fn consensus(&self, sto: &dyn Store, prevblk: &dyn BlockRead, curblk: &dyn BlockRead) -> RetErr {
        impl_consensus(self, sto, prevblk, curblk)
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