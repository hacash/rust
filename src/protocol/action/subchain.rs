



/**
 * check sub chain id
 */
 ActionDefine!{
    SubmitHeightLimit : 29, (
        start  : BlockHeight
        end    : BlockHeight
    ),
    ACTLV_TOP_UNIQUE, // level
    5+5, // gas = 32
    (self, ctx, state, store, gas), // params
    false, // burn 90
    [], // req sign
    {
        let lhei = self.start.uint();
        let rhei = self.end.uint();
        let pdhei = ctx.pending_height();
        if lhei > 0 && pdhei < lhei {
            return errf!("transction must submit in height between {} and {}", lhei, rhei)
        }
        if rhei > 0 && pdhei > lhei {
            return errf!("transction must submit in height between {} and {}", lhei, rhei)
        }
        // ok
        Ok(vec![])
    }
}




/**
 * check sub chain id
 */
 ActionDefine!{
    SubChainID : 30, (
        chain_id : Uint8
    ),
    ACTLV_TOP_UNIQUE, // level
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



