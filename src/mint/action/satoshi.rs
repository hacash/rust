
/**
 * Satoshi Transfer
 */
 ActionDefine!{
    SatoshiTransfer : 8, (
        to       : Address
        satoshi  : Satoshi
    ),
    ACTLV_TOP, // level
    21 + 8, // gas
    (self, env, state, store), // params
    false, // burn 90
    [], // req sign
    { 
        let mut state = CoreState::wrap(state);
        let from = env.main_address(); 
        ActExecRes::wrap(
            sat_transfer(&mut state, from, &self.to, &self.satoshi)
        )
    }
}



/**
 * Satoshi From To Transfer
 */
 ActionDefine!{
    SatoshiFromToTransfer : 11, (
        from     : Address 
        to       : Address 
        satoshi  : Satoshi
    ),
    ACTLV_TOP, // level
    21 + 21 + 8, // gas
    (self, env, state, store), // params
    false, // burn 90
    [self.from], // req sign
    {
        let mut state = CoreState::wrap(state);
        ActExecRes::wrap(
            sat_transfer(&mut state, &self.from, &self.to, &self.satoshi)
        )
    }
}


/**
 * Satoshi From Transfer
 */
 ActionDefine!{
    SatoshiFromTransfer : 28, (
        from     : Address 
        satoshi  : Satoshi
    ),
    ACTLV_TOP, // level
    21 + 8, // gas
    (self, env, state, store), // params
    false, // burn 90
    [self.from], // req sign
    { 
        let mut state = CoreState::wrap(state);
        let to = env.main_address(); 
        ActExecRes::wrap(
            sat_transfer(&mut state, &self.from, to, &self.satoshi)
        )
    }
}
