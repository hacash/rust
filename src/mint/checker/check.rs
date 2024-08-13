

#[derive(Clone)]
pub struct BlockMintChecker {
    cnf: MintConf,
    difficulty: DifficultyGnr,
}

impl BlockMintChecker {
    pub fn new(ini: &IniObj) -> BlockMintChecker {
        let cnf = MintConf::new(ini);
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

    fn next_difficulty(&self, prev: &dyn BlockRead, sto: &dyn Store) -> u32 {
        let pdif = prev.difficulty().uint();
        let ptim = prev.timestamp().uint();
        let nhei = prev.height().uint() + 1;
        let (difn, ..) = self.difficulty.target(&self.cnf, pdif, ptim, nhei, sto);
        difn
    }

    fn prepare(&self, sto: &dyn Store, curblk: &dyn BlockRead) -> RetErr {
        impl_prepare(self, sto, curblk)
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