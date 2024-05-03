

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

#[derive(Clone, Debug)]
struct CoinKind {
    hacash: bool,
    satoshi: bool,
    diamond: bool,
}
impl CoinKind {
    fn new(mut s: String) -> CoinKind {
        let s = s.to_lowercase();
        CoinKind {
            hacash: s.contains("h"),
            satoshi: s.contains("s"),
            diamond: s.contains("d"),
        }
    }
}

macro_rules! q_coinkind{
    ( $q: ident, $k: ident ) => (
        q_must!($q, $k, s!("hsd"));
        let $k = CoinKind::new( $k );
    )
}

macro_rules! q_unit{
    ( $q: ident, $k: ident ) => (
        q_must!($q, $k, s!("fin"));
    )
}

macro_rules! q_must{
    ( $q: ident, $k: ident, $dv: expr ) => (
        let mut $k = {
            if let Some(v) = $q.$k.clone() {
                v
            }else  {
                $dv
            }
        };
    )
}

macro_rules! defineQueryObject{
    ( $name: ident, $( $item: ident, $ty: ty, $dv: expr,)+ ) => (

        #[derive(serde::Deserialize)]
        struct $name {
            $(
                $item: $ty,
            )+
            unit: Option<String>,
            coinkind: Option<String>,
        }

        impl Default for $name {
            fn default() -> Self {
                Self { 
                    $(
                        $item: $dv,
                    )+
                    unit: None,
                    coinkind: None,
                }
            }
        }

    )
}



/*******************************/
