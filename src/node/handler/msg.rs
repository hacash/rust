
// msg types

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
    latest_height:           BlockHeight
    latest_hash:             Hash
}


