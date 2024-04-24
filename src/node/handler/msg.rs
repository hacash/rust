
pub enum BlockTxMsgStuff {
    Block(Arc<Peer>, Vec<u8>),
    Tx(Arc<Peer>, Vec<u8>),
}

// msg types

pub const MSG_TX_SUBMIT:           u16 = 7; // new tx    arrived
pub const MSG_BLOCK_DISCOVER:      u16 = 8; // new block arrived



