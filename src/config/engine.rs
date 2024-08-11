
#[derive(Clone)]
pub struct EngineConf {
    pub max_block_txs: usize,
    pub max_block_size: usize,
    pub chain_id: u64, // sub chain id
    pub unstable_block: u64, // The number of blocks that are likely to fall back from the fork
    pub fast_sync: bool,
    pub data_dir: String,
    pub store_data_dir: PathBuf, // block data
    pub state_data_dir: PathBuf, // chain state
    //
    pub miner_enable: bool,
    pub miner_reward_address: Address,
}


impl EngineConf {
    
    pub fn new(ini: &IniObj, dbv: u32) -> EngineConf {
    
        // datadir
        let data_path = get_data_path(ini);
    
        let mut state_data_dir = join_path(&data_path, "state");
        state_data_dir.push(format!("v{}", dbv));

        let mut cnf = EngineConf{
            max_block_txs: 1000,
            max_block_size: 1024*1024*1, // 1MB
            chain_id: 0,
            unstable_block: 4, // 4 block
            fast_sync: false,
            store_data_dir: join_path(&data_path, "store"),
            state_data_dir: state_data_dir,
            data_dir: data_path.to_str().unwrap().to_owned(),
            miner_enable: false,
            miner_reward_address: Address::default(),
        };

        let sec = &ini_section(ini, "node");
        cnf.fast_sync = ini_must_bool(sec, "fast_sync", false);

        let sec_mint = &ini_section(ini, "mint");
        cnf.chain_id = ini_must_u64(sec_mint, "chain_id", 0);

        let sec_miner = &ini_section(ini, "miner");
        cnf.miner_enable = ini_must_bool(sec_miner, "enable", false);
        if cnf.miner_enable {
            cnf.miner_reward_address = ini_must_address(sec_miner, "reward");
        }

        // ok
        cnf
    }
    
}
