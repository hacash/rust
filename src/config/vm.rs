
pub struct VMConf {
    pub max_gas_limit: u32,
    pub max_gas_single_act: u32,
    pub ext_act_scope: [u8; 3],
}




pub fn NewVMConf(ini: &IniObj) -> VMConf {
    let mut cnf = VMConf{
        max_gas_limit: 42_9490_0000, // 4294967296
        max_gas_single_act: 65536, // 256^2
        ext_act_scope: [0, 6, 7],
    };
    cnf
}
