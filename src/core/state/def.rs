
/**
 * block store
 */
defineChainStateOperationInstance!{
    Store, CoreStore,
    (
        &[1, 1], status        , StoreStatus
    )
    (
        &[1, 11], blockdata    , Hash        , BytesW4
        &[1, 12], blockptr     , BlockHeight , Hash
    )
}

impl CoreStoreDisk<'_> {
    pub fn blockdatabyptr(&self, hei: &BlockHeight) -> Option<BytesW4> {
        let hx = self.blockptr(hei);
        if let None = hx {
            return None // not find
        }
        self.blockdata(&hx.unwrap())
    }
}





/**
 * chain state
 */
defineChainStateOperationInstance!{
    State, CoreState,
    (
        &[1, 1], block_reward       , Amount
        &[1, 2], latest_block_intro , BlockIntro
    )
    (
        &[1, 33], txexist    , Hash    , TxExist
        &[1, 34], balance    , Address , Balance
    )
}
