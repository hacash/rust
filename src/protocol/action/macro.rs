
// : $name:ident 
#[macro_export]
macro_rules! ActionDefine {
    {   $actname:ident : $actid:expr, 
        ($( $item:ident : $type:ty )*), 
        $lv:expr, $burn90:expr, 
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

