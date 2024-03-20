

defineChainStateOperationInstance!{
    CoreState
    (
        &[0, 1], block_reward       , Amount
        // &[0, 2], latest_block_intro , BlockIntro
        // &[0, 3], latest_diamond     , DiamondSmelt
    )
    (
        // &[0, 4], tx_contain , Hash    , ContainTx
        &[0, 5], balance    , Address , Balance
    )
}

