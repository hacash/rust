
/**
 * simple hac to
 */
ActionDefine!{
    HacTransfer : 1, (
        to : Address
        amt : Amount
    ),
    ACTLV_TOP, // level
    21 + 11, // gas = 32
    (self, env, state, store), // params
    false, // burn 90
    [], // req sign
    {
        let from = env.main_address(); 
        ActExecRes::wrap(
            hac_transfer(env, state, from, &self.to, &self.amt)
        )
    }
}


/**
 * hac from
 */
 ActionDefine!{
    HacFromTransfer : 13, (
        from : Address
        amt : Amount
    ),
    ACTLV_TOP, // level
    21 + 11, // gas = 32
    (self, env, state, store), // params
    false, // burn 90
    [self.from], // req sign
    { 
        let to = env.main_address(); 
        ActExecRes::wrap(
            hac_transfer(env, state, &self.from, to, &self.amt)
        )
    }
}


/**
 * hac from to
 */
 ActionDefine!{
    HacFromToTransfer : 14, (
        from : Address
        to : Address
        amt : Amount
    ),
    ACTLV_TOP, // level
    21 + 21 + 11, // gas = 32
    (self, env, state, store), // params
    false, // burn 90
    [self.from], // req sign
    { 
        ActExecRes::wrap(
            hac_transfer(env, state, &self.from, &self.to, &self.amt)
        )
    }
}


