

defineChainStateOperationInstance!{
    CoreState
    (
        &[0, 1], block_reward       , Amount
        &[0, 2], latest_block_intro , BlockIntro
    )
    (
        &[0, 33], txexist    , Hash    , TxExist
        &[0, 33], balance    , Address , Balance
    )
}

/*
        // &[0, 3], latest_diamond     , DiamondSmelt
*/


defineChainStateOperationInstance!{
    CoreStore
    (
        &[0, 1], status        , StoreStatus
    )
    (
        &[0, 11], blockdata    , Hash        , BytesW4
        &[0, 12], blockptr     , BlockHeight , Hash
    )
}

