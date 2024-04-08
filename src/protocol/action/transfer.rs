
/**
 * simple hac to
 */
ActionDefine!{
    HacTransfer : 1, (
        to : Address
        amt : Amount
    ),
    // level
    ACTLV_TOP,
    // burn 90
    false,
    // params
    (self, env, state, store),
    // req_sign
    [],
    // exec
    { panic_never_call_this!() }
}


/**
 * hac from
 */
ActionDefine!{
    HacFromTransfer : 13, (
        from : Address
        amt : Amount
    ),
    // level
    ACTLV_TOP,
    // burn 90
    false,
    // params
    (self, env, state, store),
    // req_sign
    [self.from],
    // exec
    { panic_never_call_this!() }
}


