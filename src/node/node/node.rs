


pub struct HacashNode {
    cnf: NodeConf,
    engine: Arc<BlockEngine>,
    txpool: Arc<MemTxPool>,
    p2p: Arc<P2PManage>,
    
    tokiort: Option<TokioRuntime>,
}


impl HacashNode {

    pub fn open(ini: &IniObj, engine: Arc<BlockEngine>) -> HacashNode {
        let mut cnf = NodeConf::new(ini);

        let txpool = Arc::new(MemTxPool::new(vec![5000, 100]));
        let msghdl = Arc::new(MsgHandler::new(engine.clone(), txpool.clone()));
        let p2p = P2PManage::new(&cnf, msghdl.clone());

        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_time().enable_io()
            .build().unwrap();

        HacashNode{
            cnf: cnf,
            engine: engine,
            txpool: txpool.clone(),
            p2p: p2p.into(),
            tokiort: rt.into(),
        }
    }

}