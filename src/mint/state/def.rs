

defineChainStateOperationInstance!{
    State, MintState,
    (
        &[1, 1], total_count    , TotalCount
        &[1, 2], latest_diamond , DiamondSmelt
    )
    (
        &[1, 21], diamond       , DiamondName      , DiamondItem
        &[1, 21], diamond_ptr   , DiamondNumber    , DiamondName
    )
}


defineChainStateOperationInstance!{
    Store, MintStore,
    (
        &[1, 1], ttttttt          , Uint1
    )
    (
        &[1, 11], diamond_smelt   , DiamondName    , DiamondSmelt
    )
}

