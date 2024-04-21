
static PEER_AUTO_ID_INCREASE: AtomicU64 = AtomicU64::new(0);


#[derive(Debug)]
pub struct Peer {
    pub id: u64,
    pub pid: PeerID,
    pub name: String,
    pub is_public: bool,
    pub port: u16,
    pub addr: SocketAddr,
    // will change
    pub active: StdMutex<SystemTime>,
    pub conn_write: StdMutex<Option<OwnedWriteHalf>>,
}



impl Peer {


    pub fn nick(&self) -> String {
        let mut nick = self.name.clone();
        if self.is_public {
            nick += format!("({})", self.addr).as_str();
        }
        nick
    }

    pub fn update_active(&self) {
        *self.active.lock().unwrap() = SystemTime::now();
    }

    pub async fn disconnect(&self) {
        // drop conn obj to close
        if let Some(mut w) = self.conn_write.lock().unwrap().take() {
            tcp_send(&mut w, &vec![0u8,0,0,0]).await; // send close mark
            w.forget();
        }
    }

    pub async fn create_by_msg(mut stream: TcpStream, ty: u8, msg: Vec<u8>) -> Ret<(Arc<Peer>, OwnedReadHalf)> {
        let conn  = &mut stream;

        let mut addr = conn.peer_addr().unwrap();
        let mut is_public = false;
        let mut idnamebts: &[u8];
        let mut oginport: u16 = 0;
        if msg.len() < 4 {
            return errf!("msg length too short")
        }
        if MSG_REPORT_PEER == ty {
            oginport = u16::from_be_bytes( bufcut!(msg, 2, 4) );
            idnamebts = &msg[4..];
        }else if MSG_ANSWER_PEER == ty {
            is_public = true; // connect to public
            idnamebts = &msg[..];
        }else{
            // unsupport msg ty
            return errf!("unsupport msg ty {}", ty)
        }
        if idnamebts.len() < 32 {
            return errf!("msg length too short")
        }
        let pid = bufcut!(idnamebts, 0, PEER_ID_SIZE);
        let name = Fixed16::cons( bufcut!(idnamebts, PEER_ID_SIZE, PEER_ID_SIZE*2) ).to_readable().replace(" ", "");

        // dial to check is public ip
        if MSG_REPORT_PEER == ty {
            let mut pubaddr = addr.clone();
            pubaddr.set_port(oginport);
            if let Ok(pb) = tcp_dial_to_check_is_public_id(pubaddr, &pid, 4).await {
                if pb {
                    is_public = true; // public connect to me
                    println!("public connect to me!!!")
                }
            }
        }
        
        // conn split
        let (mut read_half, mut write_half) = stream.into_split();

        // cid
        PEER_AUTO_ID_INCREASE.fetch_add(1, Ordering::Relaxed);
        let atid = PEER_AUTO_ID_INCREASE.load(Ordering::Relaxed);

        // create
        let mut peer = Peer {
            id: atid,
            pid: pid,
            name: name,
            is_public: is_public,
            port: oginport,
            addr: addr,
            active: SystemTime::now().into(),
            conn_write: Some(write_half).into(),
        };
        let pptr = Arc::new(peer);

        // println!("create peer {} successfully: {:?}", peer.desc(), peer);
        Ok((pptr, read_half))
    }


}