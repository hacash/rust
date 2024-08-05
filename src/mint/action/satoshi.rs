
/**
 * Satoshi Transfer
 */
 ActionDefine!{
    SatoshiTransfer : 8, (
        to       : AddrOrPtr
        satoshi  : Satoshi
    ),
    ACTLV_MAIN, // level
    21 + 8, // gas
    (self, ctx, state, store, gas), // params
    false, // burn 90
    [], // req sign
    { 
        let mut state = CoreState::wrap(state);
        let from = ctx.main_address().clone(); 
        let to = self.to.real(ctx.addr_list())?;
        sat_transfer(ctx, &mut state, &from, &to, &self.satoshi)
    }
}



/**
 * Satoshi From To Transfer
 */
 ActionDefine!{
    SatoshiFromToTransfer : 11, (
        from     : AddrOrPtr 
        to       : AddrOrPtr 
        satoshi  : Satoshi
    ),
    ACTLV_MAIN, // level
    21 + 21 + 8, // gas
    (self, ctx, state, store, gas), // params
    false, // burn 90
    [self.from], // req sign
    {
        let mut state = CoreState::wrap(state);
        let from = self.from.real(ctx.addr_list())?;
        let to = self.to.real(ctx.addr_list())?;
        sat_transfer(ctx, &mut state, &from, &to, &self.satoshi)
    }
}


/**
 * Satoshi From Transfer
 */
 ActionDefine!{
    SatoshiFromTransfer : 28, (
        from     : AddrOrPtr
        satoshi  : Satoshi
    ),
    ACTLV_MAIN, // level
    21 + 8, // gas
    (self, ctx, state, store, gas), // params
    false, // burn 90
    [self.from], // req sign
    { 
        let mut state = CoreState::wrap(state);
        let from = self.from.real(ctx.addr_list())?;
        let to = ctx.main_address().clone();
        sat_transfer(ctx, &mut state, &from, &to, &self.satoshi)
    }
}
