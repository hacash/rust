
#[derive(Clone)]
pub struct PeerMngInst {
    p2p: Arc<P2PManage>,
}

impl PeerMngInst {
    pub fn new(p2p: Arc<P2PManage>) -> PeerMngInst {
        PeerMngInst{
            p2p: p2p,
        }
    }
}



impl PeerManage for PeerMngInst {

    fn switch_peer(&self, p: Arc<Peer>) -> Arc<Peer> {
        self.p2p.switch_peer(p)
    }

}



impl PeerManage for P2PManage {

    fn switch_peer(&self, p: Arc<Peer>) -> Arc<Peer> {
        let pubps = self.publics();
        if pubps.len() == 0 {
            return p
        }
        for pr in &pubps {
            if pr.id == p.id {
                return p
            }
        }
        // the first one
        pubps[0].clone()
    }

}