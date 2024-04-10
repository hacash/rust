
pub struct EngineConf {
    pub max_block_txs: usize,
    pub max_block_size: usize,
    pub unstable_block: u64, // The number of blocks that are likely to fall back from the fork
    pub store_data_dir: Box<Path>, // block data
    pub state_data_dir: Box<Path>, // chain state
    pub ctrkv_data_dir: Box<Path>, // contract kv storage

}




pub fn NewKernelConf(ini: &IniObj) -> EngineConf {

    let mut data_dir = "./hacash_mainnet_data".to_string();

    if let Some(cnfsec) = ini.get("kernel") {
        if let Some(Some(dtdir)) = cnfsec.get("data_dir") {
            data_dir = dtdir.clone();
        }
    }
    

    let mut cnf = EngineConf{
        max_block_txs: 999,
        max_block_size: 1024*1024*1, // 1MB
        unstable_block: 4, // 4 block
        store_data_dir: json_path(&data_dir, "store"),
        state_data_dir: json_path(&data_dir, "state"),
        ctrkv_data_dir: json_path(&data_dir, "ctrkv"),
    };


    cnf
}
