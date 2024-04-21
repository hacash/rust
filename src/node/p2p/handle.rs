

impl P2PManage {

    async fn handle_peer_message(&self, peer: Arc<Peer>, mut conn_read: OwnedReadHalf) -> RetErr {

        let peer1 = peer.clone();
        let peer2 = peer.clone();
        let peer3 = peer.clone();
        let pary1 = self.offshoots.clone();
        let pary2 = self.backbones.clone();
        let hdl1 = self.msghandler.clone();
        let hdl2 = self.msghandler.clone();
        let hdl3 = self.msghandler.clone();
        // on connect
        tokio::spawn(async move {
            hdl1.on_connect(peer1).await
        });
        tokio::spawn(async move {
            // handle msg
            do_handle_pmsg(pary1, pary2, hdl2, peer2, conn_read).await;
            // on disconnect
            let hdlcp = hdl3;
            tokio::spawn(async move {
                hdlcp.on_disconnect(peer3).await
            });
        });
        Ok(())
    }

}

async fn do_handle_pmsg(pary1: PeerList, pary2: PeerList, msghdl: Arc<MsgHandler>, peer: Arc<Peer>, mut conn_read: OwnedReadHalf) {

    loop {
        let rdres = tcp_read_msg(&mut conn_read).await;
        if let Err(_) = rdres {
            break // closed
        }
        peer.update_active();
        let (ty, msg) = rdres.unwrap();

        if MSG_CUSTOMER == ty {
            // on customer message
            let prcp = peer.clone();
            let ty = u16::from_be_bytes( bufcut!(msg,0,2) );
            let body = msg[2..].to_vec();
            let msghd1 = msghdl.clone();
            tokio::spawn(async move {
                msghd1.on_message(prcp, ty, body).await
            });
            continue
        }

        println!("=== Peer {} msg {} === {}", peer.nick(), ty, hex::encode(msg));
    }

    // 
    println!("--- drop the Peer {}", peer.nick());
    // close the conn
    peer.disconnect();


}