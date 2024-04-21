
/////////////////// DIAL ///////////////////


pub async fn tcp_dial_to_check_is_public_id(addr: SocketAddr, pid: &PeerID, outsec: u64) -> Ret<bool> {
    let mut conn = errunbox!( TcpStream::connect(addr).await )?;
    let conn = &mut conn;
    tcp_check_handshake(conn, 4).await?;
    tcp_send_msg(conn, MSG_REQUEST_NODE_ID_FOR_PUBLIC_CHECK, &vec![]).await?;
    let retmsg = tcp_read_timeout(conn, PEER_ID_SIZE, 3).await?;
    if *pid != *retmsg {
        return errf!("peer id not match") // false
    }
    Ok(true)
}


/////////////////// READ WRITE ///////////////////


pub async fn tcp_check_handshake(conn: &mut (impl AsyncRead + AsyncWrite + Unpin), outsec: u64) -> RetErr {
    // send handshake
    let handshake = P2P_HAND_SHAKE_MAGIC_NUMBER.to_be_bytes();
    errunbox!( AsyncWriteExt::write_all(conn, &handshake).await )?;
    // check magic
    let magicn = tcp_read_timeout(conn, 4, outsec).await?;
    if magicn != handshake {
        return errf!("tcp handshake magic number error")
    }
    Ok(())
}

pub async fn tcp_handshake_read_one_msg(conn: &mut (impl AsyncRead + AsyncWrite + Unpin), outsec: u64) -> Ret<(u8, Vec<u8>)> {
    // check handshake
    tcp_check_handshake(conn, outsec+2).await?;
    // read msg
    tcp_read_msg_timeout(conn, outsec).await
}


/////////////////// WRITE ///////////////////


pub async fn tcp_send_msg(conn: &mut (impl AsyncWrite + Unpin), msgty: u8, msgbody: &Vec<u8>) -> RetErr {

    let msgsize: u32 = 1 + msgbody.len() as u32;
    let mut bufcon = Vec::with_capacity(4 + msgsize as usize);
    bufcon.append(&mut msgsize.to_be_bytes().to_vec());
    bufcon.push(msgty);
    bufcon.append(&mut msgbody.clone());
    tcp_send(conn, &bufcon).await
}

pub async fn tcp_send(conn: &mut (impl AsyncWrite + Unpin), body: &Vec<u8>) -> RetErr {
    errunbox!( AsyncWriteExt::write_all(conn, body).await )
}


/////////////////// READ ///////////////////


// return  
pub async fn tcp_read_msg(conn: &mut (impl AsyncRead + Unpin)) -> Ret<(u8, Vec<u8>)> {
    tcp_read_msg_timeout(conn, u64::MAX).await
}

pub async fn tcp_read_msg_timeout(conn: &mut (impl AsyncRead + std::marker::Unpin), outsec: u64) -> Ret<(u8, Vec<u8>)> {
    let size = tcp_read_timeout(conn, 4, outsec).await?;
    let size = u32::from_be_bytes( bufcut!(size, 0, 4) );
    if size < 1 || size > P2P_MSG_DATA_MAX_SIZE {
        return errf!("tcp msg size error")
    }
    let tybody = tcp_read_timeout(conn, size as usize, outsec).await?;
    // ok
    Ok((tybody[0], tybody[1..].to_vec()))
}


pub async fn tcp_read_timeout(conn: &mut (impl AsyncRead + std::marker::Unpin), readlen: usize, outsec: u64) -> Ret<Vec<u8>> {

    let mut buf = vec![0; readlen];
    // no time out
    if outsec == 0 || outsec == u64::MAX {
        let rn = errunbox!( AsyncReadExt::read_exact(conn, &mut buf).await )?;
        if rn != readlen {
            return errf!("tcp read size fail")
        }
        return Ok((buf))
    }
    // set time out
    let res = match tokio::try_join!(async{
        let rn = errunbox!( AsyncReadExt::read_exact(conn, &mut buf).await )?;
        if rn != readlen {
            return errf!("tcp read size fail")
        }
        errf!("ok")
    }, async{
        asleep(outsec).await;
        errf!("tcp read timeout {}s", outsec)
    }) {
        Ok(((),())) => "".to_string(),
        Err(e) => e,
    };

    // finish
    match res.as_str() {
        "ok" => Ok(buf),
        _ => Err(res),
    }
}