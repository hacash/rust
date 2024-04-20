

/******* pubFnRegActions ********/


macro_rules! pubFnRegActionCreateCommonEx {
    ( $trycreatefn:ident, $createfn:ident, $retty:ident, $($ty:ident)+ ) => {

        pub fn $trycreatefn(kind: u16, buf: &[u8]) -> Ret<Option<(Box<dyn $retty>, usize)>> {
            $(   
            if kind == <$ty>::kid() {
                let (act, sk) = <$ty>::create(buf)?;
                return Ok(Some((Box::new(act), sk)))
            }
            )+
            Ok(None)
        }

        pub fn $createfn(buf: &[u8]) -> Ret<(Box<dyn $retty>, usize)> {
            let kid = cut_kind(buf)?;
            let hasact = $trycreatefn(kid, buf)?;
            match hasact {
                Some(res) => Ok(res),
                None => Err(format!("Action Kind <{}> not find.", kid))
            }
        }

    }
}


/******* Create Func Define ********/


#[macro_export]
macro_rules! pubFnRegActionCreates {
    ( $($ty:ident)+ ) => {

        pub fn cut_kind(buf: &[u8]) -> Ret<u16> {
            let mut kind = Uint2::new();
            kind.parse(buf, 0)?;
            let kid = kind.to_u16();
            Ok(kid)
        }

        pubFnRegActionCreateCommonEx!{
            try_create, create, Action, $($ty)+
        }

        pubFnRegActionCreateCommonEx!{
            try_create_vm, create_vm, VMAction, $($ty)+
        }

    }
}


/******* ActionDefine ********/

#[macro_export]
macro_rules! ActionDefineWithStruct {
    {   $actname:ident : $actid:expr, 
        $lv:expr, $gas:expr,
        ($p_self:ident, $p_env:ident, $p_state:ident, $p_store:ident ), 
        $burn90:expr, $reqsign:expr, $exec:expr 
    } => {

// ACTION_KIND DEFINE
concat_idents!(ACTION_KIND_ID = ACTION_KIND_, $actid {
pub const ACTION_KIND_ID: u16 = $actid;
});


impl VMAction for $actname {
    fn kind(&self) -> u16 {
        $actid
    }
    fn gas(&self) -> u32 {
        $gas
    }
    fn as_vm(&self) -> &dyn VMAction {
        self
    }
    fn as_ext(&self) -> &dyn Action {
        self
    }
}

impl ActExec for $actname {
    fn execute(&$p_self, $p_env: &dyn ExecEnv, $p_state: &mut dyn State, $p_store: &dyn Store) -> Box<dyn ExecResult> {
        $exec
    }
}


impl Action for $actname {
    fn level(&self) -> u8 {
        $lv
    }
    fn burn_90(&$p_self) -> bool { 
        $burn90
    }
    fn req_sign(&$p_self) -> HashSet<Address> {
        HashSet::from($reqsign)
    }
}

impl $actname {

    pub fn kid() -> u16 {
        $actid
    }

    pub fn new() -> $actname {
        let mut obj = <$actname as Field>::new();
        obj.kind = Uint2::from($actid);
        obj
    }
} 



/*
pub struct ACTION_NAME {
    pub kind: Uint2,
}
 */





    }
}


#[macro_export]
macro_rules! ActionDefine {
    {   $actname:ident : $actid:expr, 
        ($( $item:ident : $type:ty )*), 
        $lv:expr, $gas:expr,
        ($p_self:ident, $p_env:ident, $p_state:ident, $p_store:ident ), 
        $burn90:expr, $reqsign:expr, $exec:expr 
    } => {

        StructFieldStruct!{ $actname ,
            kind: Uint2
            $( 
                $item : $type 
            )*
        }

        ActionDefineWithStruct!{
            $actname : $actid, 
            $lv, $gas,
            ($p_self, $p_env, $p_state, $p_store ), 
            $burn90, $reqsign, $exec 
        }

    }
}