

impl P2PManage {

    pub async fn connect_boot_nodes(& self) -> RetErr {

        print!("[P2P] Connect {} boot nodes:", self.cnf.boot_nodes.len());
        for ndip in &self.cnf.boot_nodes {
            let addr = ndip.clone();
            print!(" {}...", &addr);
            if let Err(e) = self.connect_node(addr).await {
                print!("[Error: {}]", e);
            }else{
                print!("ok");
            }
        }
        println!(".");
        Ok(())
    }

    pub async fn connect_node(&self, addr: SocketAddr) -> RetErr {
        let mut conn = errunbox!( TcpStream::connect(addr).await )?;
        self.handle_conn(conn).await
    }

    pub async fn handle_conn(&self, mut conn: TcpStream) -> RetErr {
        tcp_check_handshake(&mut conn, 7).await?;
        // report my node info: mark+port+id+name
        let nodeinfo = self.pick_my_node_info();
        errunbox!( tcp_send_msg(&mut conn, MSG_REPORT_PEER, &nodeinfo).await )?;
        // deal conn
        self.insert_peer(conn).await
    }

    pub async fn insert_peer(&self, mut conn: TcpStream) -> RetErr {
        let (peer, conn_read) = self.try_create_peer(conn).await?;
        // loop read peer msg
        self.handle_peer_message(peer.clone(), conn_read).await?;
        // add to node list
        let mypid = &self.cnf.node_id;
        let mut lmax = self.cnf.offshoot_peers;
        let mut list = self.offshoots.clone();
        if peer.is_public {
            // add in backbones
            lmax = self.cnf.backbone_peers;
            list = self.backbones.clone();
        }
        let droped = insert_peer_to_dht_list(list, lmax, mypid, peer);
        if let Some(peer) = droped {
            // disconnect and drop peer
            peer.disconnect();
        }
        Ok(())
    }


    async fn try_create_peer(&self, mut stream: TcpStream) -> Ret<(Arc<Peer>, OwnedReadHalf)> {
        let conn = &mut stream;
        // read first msg
        let (ty, body) = tcp_read_msg_timeout(conn, 4).await?;
        if MSG_REMIND_ME_IS_PUBLIC == ty {
            return errf!("ok") // normal close

        }else if MSG_REQUEST_NODE_ID_FOR_PUBLIC_CHECK == ty {
            let buf = self.cnf.node_id.clone();
            AsyncWriteExt::write_all(conn, &buf).await;
            return errf!("ok") // normal close

        }else if MSG_REQUEST_NEAREST_PUBLIC_NODES == ty {
            // TODO:: 
            return errf!("ok") // normal close

        }
        // other msg
        Peer::create_by_msg(stream, ty, body).await
    }
    

    fn pick_my_node_info(&self) -> Vec<u8> {
        let mut nodeinfo = vec![0u8; 2+2+PEER_ID_SIZE*2];
        // port
        nodeinfo.splice(2..4, self.cnf.listen.to_be_bytes());
        // id
        nodeinfo.splice(4..20, self.cnf.node_id);
        //name
        let mut namebt = self.cnf.node_name.clone();
        namebt += "                ";
        namebt.truncate(PEER_ID_SIZE); // node name max 16
        nodeinfo.splice(20..20+PEER_ID_SIZE, namebt.into_bytes());
        // ok
        nodeinfo.to_vec()
    }

}
