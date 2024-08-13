
#[derive(Clone)]
pub struct PoWorkConf {
    pub rpcaddr: String,
    pub supervene: u32, // cpu core
    pub noncemax: u32,
    pub noticewait: u64, // new block notice wait
}



impl PoWorkConf {

    pub fn new(ini: &IniObj) -> PoWorkConf {
        let sec = &ini_section(ini, "default"); // default = root
    
        let mut cnf = PoWorkConf{
            rpcaddr: ini_must(sec, "connect", "127.0.0.1:8081"),
            supervene: ini_must_u64(sec, "supervene", 2) as u32,
            noncemax: ini_must_u64(sec, "nonce_max", u32::MAX as u64) as u32,
            noticewait: ini_must_u64(sec, "notice_wait", 45),
        };
    
        cnf
    }

}



