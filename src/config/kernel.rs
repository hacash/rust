
pub struct KernelConf {
    pub max_block_txs: usize,
    pub max_block_size: usize,
    pub unstable_block: u64, // The number of blocks that are likely to fall back from the fork
    pub store_data_dir: Box<Path>, // block data
    pub state_data_dir: Box<Path>, // chain state
}




pub fn NewKernelConf(ini: &IniObj) -> KernelConf {

    let mut data_dir = "./hacash_mainnet_data".to_string();

    let cnfsec = &ini["kernel"];
    if let Some(dtdir) = &cnfsec["datadir"] {
        data_dir = dtdir.clone();
    }
    

    let mut cnf = KernelConf{
        max_block_txs: 999,
        max_block_size: 1024*1024*1, // 1MB
        unstable_block: 4, // 4 block
        store_data_dir: json_path(&data_dir, "store"),
        state_data_dir: json_path(&data_dir, "state"),
    };


    cnf
}
