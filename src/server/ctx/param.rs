

#[macro_export]
macro_rules! ctx_state{
    ($ctx:expr, $state:ident) => (
        let _s1_db = $ctx.engine.state();
        let $state = CoreStateDisk::wrap(_s1_db.as_ref());
    )
}

#[macro_export]
macro_rules! ctx_store{
    ($ctx:expr, $store:ident) => (
        let _s2_db = $ctx.engine.store();
        let $store = CoreStoreDisk::wrap(_s2_db.as_ref());
    )
}

#[macro_export]
macro_rules! ctx_mintstate{
    ($ctx:expr, $mintstate:ident) => (
        let _s3_db = $ctx.engine.state();
        let $mintstate = MintStateDisk::wrap(_s3_db.as_ref());
    )
}

#[macro_export]
macro_rules! ctx_mintstore{
    ($ctx:expr, $mintstore:ident) => (
        let _s4_db = $ctx.engine.store();
        let $mintstore = MintStoreDisk::wrap(_s4_db.as_ref());
    )
}

#[derive(Clone, Debug)]
pub struct CoinKind {
    pub hacash: bool,
    pub satoshi: bool,
    pub diamond: bool,
    pub diamonds: bool,
}
impl CoinKind {
    pub fn new(mut s: String) -> CoinKind {
        let s = s.to_lowercase();
        CoinKind {
            hacash: s.contains("h"),
            satoshi: s.contains("s"),
            diamond: s.contains("d"),
            diamonds: s.contains("n"),
        }
    }
}

#[macro_export]
macro_rules! q_coinkind{
    ( $q: ident, $k: ident ) => (
        q_must!($q, $k, s!("hsd"));
        let $k = CoinKind::new( $k );
    )
}

#[macro_export]
macro_rules! q_unit{
    ( $q: ident, $k: ident ) => (
        q_must!($q, $k, s!("fin"));
    )
}

#[macro_export]
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

#[macro_export]
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
