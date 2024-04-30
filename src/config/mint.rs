
#[derive(Clone, Copy)]
pub struct MintConf {
    pub difficulty_adjust_blocks: u64, // height
    pub each_block_target_time: u64, // secs
    pub _test_mul: u64,
}




pub fn NewMintConf(ini: &IniObj) -> MintConf {

    let sec = ini_section(ini, "mint");

    let mut cnf = MintConf{
        difficulty_adjust_blocks: ini_must_u64(&sec, "difficulty_adjust_blocks", 288), // 1 day
        each_block_target_time: ini_must_u64(&sec, "each_block_target_time", 300), // 5 mins
        _test_mul: ini_must_u64(&sec, "_test_mul", 1), // test
    };

    cnf
}
