

pub struct MsgHandler {
    blktxch: Sender<BlockTxArrive>,
    engine: Arc<BlockEngine>,
    txpool: Arc<MemTxPool>,
}


impl MsgHandler {

    pub fn new(blktxch: Sender<BlockTxArrive>, engine: Arc<BlockEngine>, txpool: Arc<MemTxPool>) -> MsgHandler {
        MsgHandler{
            blktxch: blktxch,
            engine: engine,
            txpool: txpool,
        }
    }

    pub async fn on_connect(&self, peer: Arc<Peer>) {
        // println!("on_connect peer={}", peer.nick());
    }
    
    pub async fn on_disconnect(&self, peer: Arc<Peer>) {
        // println!("on_disconnect peer={}", peer.nick());
        // do nothing
    }
    
    pub async fn on_message(&self, peer: Arc<Peer>, ty: u16, msgbody: Vec<u8>) {

        if MSG_TX_SUBMIT == ty {
            self.blktxch.send(BlockTxArrive::Tx(peer.clone(), msgbody)).await;
            return
        }
        if MSG_BLOCK_DISCOVER == ty {
            self.blktxch.send(BlockTxArrive::Block(peer.clone(), msgbody)).await;
            return
        }

        // println!("on_message peer={} ty={}  body={}", peer.nick(), ty, hex::encode(msgbody));
    }


}