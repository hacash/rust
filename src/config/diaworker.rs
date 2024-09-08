
#[derive(Clone)]
pub struct DiaWorkConf {
    pub rpcaddr: String,
    pub supervene: u32, // cpu core
    pub bidaddr: Address,
    pub rewardaddr: Address,
}


impl DiaWorkConf {

    pub fn new(ini: &IniObj) -> DiaWorkConf {
        let sec = &ini_section(ini, "default"); // default = root
    
        let mut cnf = DiaWorkConf{
            rpcaddr: ini_must(sec, "connect", "127.0.0.1:8081"),
            supervene: ini_must_u64(sec, "supervene", 2) as u32,
            bidaddr: Address::default(),
            rewardaddr: Address::default(),
        };
    
        cnf
    }

}



