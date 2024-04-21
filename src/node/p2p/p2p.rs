type PeerList = Arc<StdMutex<Vec<Arc<Peer>>>>;


pub struct P2PManage {
    cnf: NodeConf,
    msghandler: Arc<MsgHandler>,
    // 
    backbones: PeerList, // 4
    offshoots: PeerList, // 200
}

impl P2PManage {

    pub fn new(cnf: &NodeConf, msghl: Arc<MsgHandler>) -> P2PManage {
        P2PManage {
            cnf: cnf.clone(),
            msghandler: msghl,
            backbones: StdMutex::new(vec![]).into(),
            offshoots: StdMutex::new(vec![]).into(),
        }
    }

}