

static SYNCING_MARK: AtomicBool = AtomicBool::new(false);

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
        self.peermng.lock().unwrap().as_ref().unwrap().switch_peer(p)
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
    
    pub async fn on_message(&self, peer: Arc<Peer>, ty: u16, body: Vec<u8>) {
        // println!("on_message peer={} ty={} len={}", peer.nick(), ty, body.len());

        match ty {
            MSG_TX_SUBMIT =>      { self.blktxch.send(BlockTxArrive::Tx(peer.clone(), body)).await; },
            MSG_BLOCK_DISCOVER => { self.blktxch.send(BlockTxArrive::Block(peer.clone(), body)).await; },
            MSG_BLOCK_HASH =>     { self.receive_hashs(peer, body).await; },
            MSG_REQ_BLOCK_HASH => { self.send_hashs(peer, body).await; },
            MSG_BLOCK =>          { self.receive_blocks(peer, body).await; },
            MSG_REQ_BLOCK =>      { self.send_blocks(peer, body).await; },
            MSG_REQ_STATUS =>     { self.send_status(peer).await; },
            MSG_STATUS =>         { self.receive_status(peer, body).await; },
            _ => /* not find msg type and ignore */ (),
        };

    }


}