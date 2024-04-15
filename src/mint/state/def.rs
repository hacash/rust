
/**
 * block store
 */
 defineChainStateOperationInstance!{
    Store, MintStore,
    (
        &[1, 1], testttt          , Uint1
    )
    (
        &[1, 11], diamond_smelt   , DiamondName    , DiamondSmelt
    )
}



/**
 * chain state
 */
defineChainStateOperationInstance!{
    State, MintState,
    (
        &[1, 1], total_count    , TotalCount
        &[1, 2], latest_diamond , DiamondSmelt
    )
    (
        &[1, 21], diamond_ptr   , DiamondNumber    , DiamondName
        &[1, 22], diamond       , DiamondName      , DiamondSto
        &[1, 23], channel       , ChannelId        , ChannelSto
    )
}

