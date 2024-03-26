
pub struct VMConf {
    pub max_gas_limit: u32,
}




pub fn NewVMConf(ini: &IniObj) -> VMConf {
    let mut cnf = VMConf{
        max_gas_limit: 42_9490_0000,
    };
    cnf
}
