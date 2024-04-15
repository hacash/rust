




/**
 * Channel Open
 */
 ActionDefine!{
    ChannelOpen : 2, (
        channel_id     : ChannelId
        left_bill      : AddrHac
        right_bill     : AddrHac
    ),
    ACTLV_TOP_ONLY, // level
    16 + (21+11)*2, // gas
    (self, env, state, store), // params
    false, // burn 90
    [], // req sign
    {
        // let from = env.main_address(); 
        // ActExecRes::wrap(
        //     hac_transfer(env, state, from, &self.to, &self.amt)
        // )
        Box::new(ActExecRes::new())
    }
}
