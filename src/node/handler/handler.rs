

pub struct MsgHandler {
    engine: Arc<BlockEngine>,
    txpool: Arc<MemTxPool>,
}


impl MsgHandler {

    pub fn new(engine: Arc<BlockEngine>, txpool: Arc<MemTxPool>) -> MsgHandler {
        MsgHandler{
            engine: engine,
            txpool: txpool,
        }
    }

    pub async fn on_connect(&self, peer: Arc<Peer>) {
        // println!("on_connect peer={}", peer.nick());
        
    }
    
    pub async fn on_disconnect(&self, peer: Arc<Peer>) {
        println!("on_disconnect peer={}", peer.nick());
        
    }
    
    pub async fn on_message(&self, peer: Arc<Peer>, ty: u16, msgbody: Vec<u8>) {
        println!("on_message peer={} ty={}  body={}", peer.nick(), ty, hex::encode(msgbody));

    }


}