
// msg types

pub const MSG_REQ_STATUS:          u16 = 1;
pub const MSG_STATUS:              u16 = 2;

pub const MSG_REQ_BLOCK_HASH:      u16 = 3;
pub const MSG_BLOCK_HASH:          u16 = 4;

pub const MSG_REQ_BLOCK:           u16 = 5;
pub const MSG_BLOCK:               u16 = 6;

pub const MSG_TX_SUBMIT:           u16 = 7; // new tx    arrived
pub const MSG_BLOCK_DISCOVER:      u16 = 8; // new block arrived

// msg stuff

pub enum BlockTxArrive {
    Block(Arc<Peer>, Vec<u8>),
    Tx(Arc<Peer>, Vec<u8>),
}


// test
StructFieldStruct!{ HandshakeStatus,
    genesis_hash:            Hash
    block_version:           Uint1
    transaction_type:        Uint1
    action_kind:             Uint2
    repair_serial:           Uint2
    __mark:                  Uint3
    latest_height:           BlockHeight // uint5
    latest_hash:             Hash
}

// msg send

async fn send_req_block_msg(peer: Arc<Peer>, starthei: u64) {
    let hei = Uint8::from(starthei);
    peer.send_msg(MSG_REQ_BLOCK, hei.serialize()).await;
    flush!("sync block from {} {}...", peer.name(), starthei);
}

async fn send_req_block_hash_msg(peer: Arc<Peer>, num: u8, starthei: u64) {
    let hei = BlockHeight::from(starthei);
    let buf = vec![vec![num], hei.serialize()].concat();
    peer.send_msg(MSG_REQ_BLOCK, buf).await;
}
