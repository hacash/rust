
#[derive(Clone)]
pub struct RPCServer {
    cnf: ServerConf,
    engine: ChainEngine,
}


impl RPCServer {
    pub fn open(iniobj: &IniObj, engine: ChainEngine) -> RPCServer {
        let cnf = NewServerConf(iniobj);
        RPCServer{
            cnf: cnf,
            engine: engine,
        }
    }

}

