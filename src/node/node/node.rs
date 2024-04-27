


pub struct HacashNode {
    cnf: NodeConf,
    engine: Arc<BlockEngine>,
    txpool: Arc<MemTxPool>,
    p2p: Arc<P2PManage>,
    
    blktxch: Option<Receiver<BlockTxArrive>>,

    knows: Knowledge,

    // close mark
    closech: StdMutex<Option<mpsc::Receiver<bool>>>,
    closechtx: mpsc::Sender<bool>,
}


impl HacashNode {

    pub fn open(ini: &IniObj, engine: Arc<BlockEngine>) -> HacashNode {
        let mut cnf = NodeConf::new(ini);

        let (closetx, closerx) = mpsc::channel(5);
        let (tx, rx): (Sender<BlockTxArrive>, Receiver<BlockTxArrive>) = mpsc::channel(4000);

        let txpool = Arc::new(MemTxPool::new(vec![5000, 100]));
        let msghdl = Arc::new(MsgHandler::new(tx.clone(), engine.clone(), txpool.clone()));
        let p2p = Arc::new(P2PManage::new(&cnf, msghdl.clone()));
        msghdl.set_peer_mng(Box::new(PeerMngInst::new(p2p.clone())));


        HacashNode{
            cnf: cnf,
            engine: engine,
            txpool: txpool.clone(),
            p2p: p2p,
            blktxch: rx.into(),
            knows: Knowledge::new(100),
            // close mark
            closech: Some(closerx).into(),
            closechtx: closetx,
        }
    }

    pub fn close(this: Arc<HacashNode>) {
        P2PManage::close(this.p2p.clone());
        new_current_thread_tokio_rt().block_on(async move {
            this.closechtx.send(true).await; // close
        });
    }

}