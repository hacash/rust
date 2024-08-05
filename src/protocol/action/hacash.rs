
/**
 * simple hac to
 */
ActionDefine!{
    HacTransfer : 1, (
        to : AddrOrPtr
        amt : Amount
    ),
    ACTLV_MAIN, // level
    21 + 11, // gas = 32
    (self, ctx, state, store, gas), // params
    false, // burn 90
    [], // req sign
    {
        let from = ctx.main_address().clone(); 
        let to = self.to.real(ctx.addr_list())?;
        hac_transfer(ctx, state, &from, &to, &self.amt)
    }
}


/**
 * hac from
 */
 ActionDefine!{
    HacFromTransfer : 13, (
        from : AddrOrPtr
        amt : Amount
    ),
    ACTLV_MAIN, // level
    21 + 11, // gas = 32
    (self, ctx, state, store, gas), // params
    false, // burn 90
    [self.from], // req sign
    {
        let from = self.from.real(ctx.addr_list())?;
        let to = ctx.main_address().clone(); 
        hac_transfer(ctx, state, &from, &to, &self.amt)
    }
}


/**
 * hac from to
 */
 ActionDefine!{
    HacFromToTransfer : 14, (
        from : AddrOrPtr
        to : AddrOrPtr
        amt : Amount
    ),
    ACTLV_MAIN, // level
    21 + 21 + 11, // gas = 32
    (self, ctx, state, store, gas), // params
    false, // burn 90
    [self.from], // req sign
    { 
        let from = self.from.real(ctx.addr_list())?;
        let to = self.to.real(ctx.addr_list())?;
        hac_transfer(ctx, state, &from, &to, &self.amt)
    }
}


