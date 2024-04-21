

impl P2PManage {

    async fn handle_peer_message(&self, peer: Arc<Peer>, mut conn_read: OwnedReadHalf) -> RetErr {

        let peer1 = peer.clone();
        let peer2 = peer.clone();
        let peer3 = peer.clone();
        let pary1 = self.offshoots.clone();
        let pary2 = self.backbones.clone();
        let hdl1 = self.msghandler.clone();
        let hdl2 = self.msghandler.clone();
        // on connect
        tokio::spawn(async move {
            hdl1.on_connect(peer1).await
        });
        tokio::spawn(async move {
            // handle msg
            do_handle_pmsg(pary1, pary2, peer3, conn_read).await;
            // // on disconnect
            // tokio::spawn(async move {
            //     hdl2.on_disconnect(peer2).await
            // });
        });
        Ok(())
    }

}

async fn do_handle_pmsg(pary1: PeerList, pary2: PeerList, peer: Arc<Peer>, mut conn_read: OwnedReadHalf) {

    loop {
        let rdres = tcp_read_msg(&mut conn_read).await;
        if let Err(_) = rdres {
            break // closed
        }
        let (ty, msg) = rdres.unwrap();

        println!("=== Peer {} msg {} === {}", peer.nick(), ty, hex::encode(msg));

    }

    // 
    println!("--- drop the Peer {}", peer.nick());
    // close the conn
    peer.disconnect();


}