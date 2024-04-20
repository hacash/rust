


pub struct HacashNode {
    cnf: NodeConf,
    engine: Arc<BlockEngine>,
    tokiort: Option<TokioRuntime>,
    
    // sigl
    mk: u32,

    p2p: P2PManage,
    
}


impl HacashNode {

    pub fn open(ini: &IniObj, engine: Arc<BlockEngine>) -> HacashNode {
        let mut cnf = NodeConf::new(ini);
        let p2p = P2PManage::new(&cnf);
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_time().enable_io()
            .build().unwrap();
        HacashNode{
            cnf: cnf,
            engine: engine,
            tokiort: Some(rt),
            p2p: p2p,
            // 
            mk: 1,

        }
    }

}