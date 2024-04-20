


pub struct HacashNode {
    cnf: NodeConf,
    engine: Arc<BlockEngine>,
    p2p: Arc<P2PManage>,
    
    tokiort: Option<TokioRuntime>,
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
            p2p: p2p.into(),

            tokiort: rt.into(),
        }
    }

}