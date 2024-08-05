



/**
 * execute script
 */
 ActionDefine!{
    ScriptExecute : 37, (
        mark: Fixed1
        vern: Fixed1
        codes: BytesW2
    ),
    ACTLV_TOP, // level
    11, // gas = 32
    (self, ctx, state, store, gas), // params
    true, // burn 90
    [], // req sign
    { 
        errf!("not support")
        /*
        let addr = Fixed21{ bytes: [0u8; 21] };
        let codes = [74u8,89];
        // ctx.vm()?.main_call(&addr, &codes)
        Ok(vec![])
        */
    }
}



