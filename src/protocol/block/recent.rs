
StructFieldStruct!{ RecentBlockInfo, 
    height:  BlockHeight
    hash:    Hash
    prev:    Hash
    txs:     Uint4 // transaction_count
    miner:   Address
    message: StringTrim16
    reward:  Amount
    time:    Timestamp
    arrive:  Timestamp
}


pub fn create_recent_block_info(blk: &dyn BlockRead) -> RecentBlockInfo {
    let coinbase = &blk.transactions()[0];
    RecentBlockInfo {
        height:  blk.height().clone(),
        hash:    blk.hash(),
        prev:    blk.prevhash().clone(),
        txs:     blk.transaction_count().clone(), // transaction_count
        miner:   coinbase.address().unwrap(),
        message: coinbase.message().clone(),
        reward:  coinbase.reward().clone(),
        time:    blk.timestamp().clone(),
        arrive:  Timestamp::from(curtimes()),
    }
}
