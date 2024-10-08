

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
}
impl CoinKind {
    pub fn new(mut s: String) -> CoinKind {
        match s.to_lowercase().as_str() {
            "all" | "hsd" => CoinKind {
                hacash: true,
                satoshi: true,
                diamond: true,
            },
            _ => CoinKind {
                hacash: s.contains("h"),
                satoshi: s.contains("s"),
                diamond: s.contains("d"),
            }
        }
    }
}

#[macro_export]
macro_rules! q_coinkind {
    ( $q: ident, $k: ident ) => (
        q_must!($q, $k, s!("hsd"));
        let $k = CoinKind::new( $k );
    )
}

#[macro_export]
macro_rules! q_unit {
    ( $q: ident, $k: ident ) => (
        q_must!($q, $k, s!("fin"));
    )
}

#[macro_export]
macro_rules! q_must {
    ( $q: ident, $k: ident, $dv: expr ) => (
        let mut $k = match $q.$k.clone() 
        {
            Some(v) => v,
            _ => $dv,
        };
    )
}

#[macro_export]
macro_rules! q_body_data_may_hex {
    ( $q: ident, $d: expr) => (
        { 
            q_must!($q, hexbody, false);
            let bddt = $d.to_vec();
            match hexbody {
                false => bddt,
                true => {
                    let res = hex::decode(&bddt);
                    if let Err(_) = res {
                        return api_error("hex format error")
                    }
                    res.unwrap()
                }
            }
        }
    )
}

#[macro_export]
macro_rules! q_addr {
    ($adr: expr) => ({
        let adr = Address::from_readable(&$adr);
        if let Err(e) = adr {
            return api_error(&format!("address {} format error: {}", &$adr, &e))
        }
        adr.unwrap()
    })
}

#[macro_export]
macro_rules! q_data_addr {
    ( $q: ident, $adr: ident) => (
        q_addr!(&$q.$adr)
    )
}

#[macro_export]
macro_rules! q_amt {
    ( $amt: expr) => ({
        let amt = Amount::from_string_unsafe(&$amt);
        if let Err(e) = amt {
            return api_error(&format!("amount {} format error: {}", &$amt, &e))
        }
        amt.unwrap()
    })
}

#[macro_export]
macro_rules! q_data_amt {
    ( $q: ident, $amt: ident) => (
        q_amt!($q.$amt)
    )
}

#[macro_export]
macro_rules! q_data_acc_from {
    ( $acc: expr) => ({
        let acc = account::Account::create_by(&$acc);
        if let Err(e) = acc {
            return api_error(&format!("prikey error: {}", &e))
        }
        acc.unwrap()
    })
}

#[macro_export]
macro_rules! q_data_acc {
    ( $q: ident, $acc: ident) => (
        q_data_acc_from!($q.$acc)
    )
}

#[macro_export]
macro_rules! q_data_hash {
    ( $hxstr: ident) => ({
        let hx = hex::decode($hxstr);
        if let Err(e) = hx {
            return api_error(&format!("hash parse error: {}", &e))
        }
        let hx = hx.unwrap();
        if hx.len() != HASH_SIZE {
            return api_error(&format!("hash size error"))
        }
        Hash::cons(hx.try_into().unwrap())
    })
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
            hexbody: Option<bool>,
            base64body: Option<bool>,
            hex: Option<bool>,
            base64: Option<bool>,
        }

        impl Default for $name {
            fn default() -> Self {
                Self { 
                    $(
                        $item: $dv,
                    )+
                    unit: None,
                    coinkind: None,
                    hexbody: None,
                    base64body: None,
                    hex: None,
                    base64: None,
                }
            }
        }

    )
}



/*******************************/
