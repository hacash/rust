
/**
 * simple hac to
 */
ActionDefine!{
    HacTransfer : 1, (
        to : Address
        amt : Amount
    ),
    ACTLV_TOP, // level
    false, // burn 90
    (self, env, state, store), // params
    [], // req_sign
    {
        let from = env.main_address(); 
        operate::hac_transfer(env, state, from, &self.to, &self.amt);
        Box::new(operate::ActExecRes::new())
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
    false, // burn 90
    (self, env, state, store), // params
    [self.from], // req_sign
    { panic_never_call_this!() }
}


