
#[derive(Clone, Copy)]
pub struct ServerConf {
    pub enable: bool,
    pub rpc_listen: u16,
    pub multi_thread: bool,
}




pub fn NewServerConf(ini: &IniObj) -> ServerConf {
    let sec = ini_section(ini, "server");
    let mut cnf = ServerConf{
        enable:       ini_must_bool(&sec, "enable", false),
        rpc_listen:   ini_must_u64(&sec, "rpc_listen", 8083) as u16,
        multi_thread: ini_must_bool(&sec, "multi_thread", false),
    };

    cnf
}

