

/******* pubFnRegActions ********/


#[macro_export]
macro_rules! pubFnRegActionCreates {
    ( $($ty:ident)+ ) => {

pub fn try_create(kind: u16, buf: &[u8]) -> Ret<Option<(Box<dyn Action>, usize)>> {
    $(   
    if kind == <$ty>::kid() {
        let (act, sk) = <$ty>::create(buf) ? ;
        return Ok(Some((Box::new(act), sk)))
    }
    )+
    Ok(None)
}

pub fn try_create_vm(kind: u16, buf: &[u8]) -> Ret<Option<(Box<dyn VMAction>, usize)>> {
    $(   
    if kind == <$ty>::kid() {
        let (act, sk) = <$ty>::create(buf) ? ;
        return Ok(Some((Box::new(act), sk)))
    }
    )+
    Ok(None)
}


pub fn cut_kind(buf: &[u8]) -> Ret<u16> {
    let mut kind = Uint2::new();
    kind.parse(buf, 0) ? ;
    let kid = kind.to_u16();
    Ok(kid)
}

pub fn create(buf: &[u8]) -> Ret<(Box<dyn Action>, usize)> {
    let kid = cut_kind(buf) ? ;
    let hasact = try_create(kid, buf) ? ;
    match hasact {
        Some(res) => Ok(res),
        None => Err(format!("Action Kind <{}> not find.", kid))
    }
}

pub fn create_vm(buf: &[u8]) -> Ret<(Box<dyn VMAction>, usize)> {
    let kid = cut_kind(buf) ? ;
    let hasact = try_create_vm(kid, buf) ? ;
    match hasact {
        Some(res) => Ok(res),
        None => Err(format!("Action Kind <{}> not find.", kid))
    }
}

    }
}


/******* ActionDefine ********/


#[macro_export]
macro_rules! ActionDefine {
    {   $actname:ident : $actid:expr, 
        ($( $item:ident : $type:ty )*), 
        $lv:expr, $burn90:expr, $gas:expr,
        ($p_self:ident, $p_env:ident, $p_state:ident, $p_store:ident ), 
        $reqsign:expr, $exec:expr 
    } => {

// ACTION_KIND DEFINE
concat_idents!(ACTION_KIND_ID = ACTION_KIND_, $actid {
pub const ACTION_KIND_ID: u16 = $actid;
});


StructFieldStruct!{ $actname ,
    kind: Uint2
    $( 
        $item : $type 
    )*
}


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
    fn burn_90(&self) -> bool { 
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
        obj.kind.parse_u16($actid);
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

