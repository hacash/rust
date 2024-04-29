

macro_rules! ctx_state{
    ($ctx:expr, $state:ident) => (
        let _s_db = $ctx.engine.state();
        let $state = CoreStateDisk::wrap(_s_db.as_ref());
    )
}
