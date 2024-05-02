

macro_rules! ctx_state{
    ($ctx:expr, $state:ident) => (
        let _s_db = $ctx.engine.state();
        let $state = CoreStateDisk::wrap(_s_db.as_ref());
    )
}

macro_rules! ctx_store{
    ($ctx:expr, $store:ident) => (
        let _s_db = $ctx.engine.store();
        let $store = CoreStoreDisk::wrap(_s_db.as_ref());
    )
}

macro_rules! q_unit{
    ( $p: ident ) => (
        q_must!($p, unit, s!("fin"))
    )
}

macro_rules! q_must{
    ( $p: ident, $k: ident, $dv: expr ) => (
        {
            if let Some(v) = $p.$k.clone() {
                v
            }else  {
                $dv
            }
        }
    )
}

macro_rules! defineQueryObject{
    ( $name: ident, $( $item: ident, $ty: ty, $dv: expr,)+ ) => (

        #[derive(serde::Deserialize)]
        struct $name {
            $(
                $item: $ty,
            )+
        }

        impl Default for $name {
            fn default() -> Self {
                Self { 
                    $(
                        $item: $dv,
                    )+
                }
            }
        }

    )
}



/*******************************/
