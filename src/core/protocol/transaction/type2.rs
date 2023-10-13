
// create new
pub fn new_type_2(addr: &Address, fee: &Amount, ts: i64) -> TransactionType2 {
    TransactionType2{
        ty:             Uint1::from_uint(2),
        timestamp:      Timestamp::from_uint(ts as u64),
        address:        addr.clone(),
        fee:            fee.clone(),
        actions:        DynListActionMax65535::new(),
        signs:          SignListMax65535::new(),
        multisign_mark: Uint2::from_uint(0),
    }
}
