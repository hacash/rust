
#[derive(Clone)]
pub struct EngineConf {
    pub max_block_txs: usize,
    pub max_block_size: usize,
    pub unstable_block: u64, // The number of blocks that are likely to fall back from the fork
    pub fast_sync: bool,
    pub data_dir: String,
    pub store_data_dir: Box<Path>, // block data
    pub state_data_dir: Box<Path>, // chain state
    pub ctrkv_data_dir: Box<Path>, // contract kv storage

}


impl EngineConf {
    
    pub fn new(ini: &IniObj) -> EngineConf {
    
        // datadir
        let data_dir = get_datadir(ini);
    
        let mut cnf = EngineConf{
            max_block_txs: 999,
            max_block_size: 1024*1024*1, // 1MB
            unstable_block: 4, // 4 block
            fast_sync: false,
            store_data_dir: join_path(&data_dir, "store"),
            state_data_dir: join_path(&data_dir, "state"),
            ctrkv_data_dir: join_path(&data_dir, "ctrkv"),
            data_dir: data_dir,
        };

        let sec = ini_section(ini, "node");
        cnf.fast_sync = ini_must_bool(&sec, "fast_sync", false);

        // ok
        cnf
    }
    
}
