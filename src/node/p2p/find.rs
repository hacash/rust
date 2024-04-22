

/**
 * Find and Connect new public node
 */
impl P2PManage {

    pub fn find_nodes(p2p: Arc<P2PManage>) {
        tokio::spawn(async move {
            do_find_nodes(p2p.as_ref()).await
        });
    }
    
}


async fn do_find_nodes(this: &P2PManage) {
    print!("[P2P] Search nodes... ");
    let mut allfindnodes = HashMap::<PeerKey, SocketAddr>::new();
    // search form backbone nodes
    let mut willdropeds = vec![ this.cnf.node_key.clone() ];
    let peers = this.clone_backbones();
    for p in peers {
        willdropeds.push(p.key.clone());
        request_public_nodes(p.addr, &mut allfindnodes).await;
    }
    if willdropeds.len() <= 1 {
        println!("not connected any nodes.");
        return
    }
    // drop myself and current connected
    for rmp in &willdropeds {
        allfindnodes.remove(rmp);
    }
    let newndcount = allfindnodes.len();
    if newndcount == 0 {
        println!("not find any new nodes.");
        return
    }
    // check nearest
    let compare = &willdropeds[0]; // my node key
    let least = &willdropeds[willdropeds.len() - 1]; // tail
    let mut nearest_list: Vec<PeerKey> = Vec::new();
    let mut mearest_addrs: Vec<SocketAddr> = Vec::new();
    for nd in &allfindnodes {
        if insert_nearest_to_dht_list(&mut nearest_list, compare, least, nd.0) {
            mearest_addrs.push(allfindnodes[nd.0].clone());
        }
    }
    println!("find {} nearest nodes, try connect... ", mearest_addrs.len());
    // try connect for each
    for addr in &mearest_addrs {
        if let Err(e) = this.connect_node(addr).await {
            println!("Fail connect to {}, {}.", addr, e);
        }
    }
    // finish
    // println!("ok do find nodes {} !!!!!!!!!!!!!!!!!!!", 1);
}



async fn request_public_nodes(addr: SocketAddr, datas: &mut HashMap<PeerKey, SocketAddr>) -> RetErr {
    let adrbts = tcp_dial_handshake_send_msg_and_read_all(
        addr, MSG_REQUEST_NEAREST_PUBLIC_NODES, 5).await?;
    let sn = 6+16; // ip port + key
    let mut num = adrbts[0] as usize;
    if num < 1 {
        return Ok(()) // not find any
    }
    if num*sn != adrbts.len()-1 {
        return errf!("data size error");
    }
    let addrs = parse_public_nodes(&adrbts[1..]);
    for p in addrs {
        datas.insert(p.0, p.1);
    }
    // ok fnish
    Ok(())
}


