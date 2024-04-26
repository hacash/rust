

static MARK_SYNCING: AtomicBool = AtomicBool::new(false);

pub struct MsgHandler {
    blktxch: Sender<BlockTxArrive>,
    engine: Arc<BlockEngine>,
    txpool: Arc<MemTxPool>,
    peermng: StdMutex<Option<Box<dyn PeerManage>>>,
}

impl MsgHandler {

    pub fn new(blktxch: Sender<BlockTxArrive>, engine: Arc<BlockEngine>, txpool: Arc<MemTxPool>) -> MsgHandler {
        MsgHandler{
            blktxch: blktxch,
            engine: engine,
            txpool: txpool,
            peermng: None.into(),
        }
    }

    pub fn switch_peer(&self, p: Arc<Peer>) -> Arc<Peer> {
        self.peermng.lock().unwrap().clone().unwrap().switch_peer(p)
    }

    pub fn set_peer_mng(&self, mng: Box<dyn PeerManage>) {
        let mut mymng = self.peermng.lock().unwrap();
        *mymng = Some(mng);
    }

}


/**
* handle message
*/
impl MsgHandler {

    pub async fn on_connect(&self, peer: Arc<Peer>) {
        // println!("on_connect peer={}", peer.nick());
        peer.send_msg(MSG_REQ_STATUS, vec![]).await;
    }
    
    pub async fn on_disconnect(&self, peer: Arc<Peer>) {
        // println!("on_disconnect peer={}", peer.nick());
        // do nothing
    }
    
    pub async fn on_message(&self, peer: Arc<Peer>, ty: u16, msgbody: Vec<u8>) {
        // println!("on_message peer={} ty={} len={}", peer.nick(), ty, msgbody.len());

        if MSG_TX_SUBMIT == ty {
            self.blktxch.send(BlockTxArrive::Tx(peer.clone(), msgbody)).await;
            return
        }
        if MSG_BLOCK_DISCOVER == ty {
            self.blktxch.send(BlockTxArrive::Block(peer.clone(), msgbody)).await;
            return
        }

        // TODO: other

        if MSG_BLOCK == ty {
            self.receive_blocks(peer, msgbody).await;
            return
        }

        if MSG_REQ_STATUS == ty {
            self.send_status(peer).await;
            return
        }

        if MSG_STATUS == ty {
            self.receive_status(peer, msgbody).await;
            return
        }

    }


}