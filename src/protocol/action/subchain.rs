/**
 * check sub chain id
 */
 ActionDefine!{
    SubChainID : 30, (
        chain_id : Uint8
    ),
    ACTLV_TOP_ONLY, // level
    8, // gas = 32
    (self, ctx, state, store, gas), // params
    false, // burn 90
    [], // req sign
    {
        let lid = ctx.chain_id();
        let sid = self.chain_id.uint();
        if lid != sid {
            return errf!("transction must belong to chain id {} but in chain {}", sid, lid)
        }
        // ok
        Ok(vec![])
    }
}

