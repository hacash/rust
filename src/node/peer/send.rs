

impl Peer {

    pub async fn send_p2p_msg(&self, ty: u8, body: Vec<u8>) -> RetErr {
        let mut w;
        {
            w = match self.conn_write.lock().unwrap().take() {
                None => return errf!("peer may be busy or closed"),
                Some(mut w) => w,
            };
        }
        tcp_send_msg(&mut w, ty, body).await;
        {
            *self.conn_write.lock().unwrap() = Some(w);
        }
        Ok(())
    }


}