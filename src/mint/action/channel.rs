
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
    21 + 11, // gas = 32
    (self, env, state, store), // params
    [], // req sign
    {
        let from = env.main_address(); 
        ActExecRes::wrap(
            hac_transfer(env, state, from, &self.to, &self.amt)
        )
    }
}
