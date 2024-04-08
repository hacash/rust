
// : $name:ident 
#[macro_export]
macro_rules! ActionDefine {
    {$actname:ident : $actid:expr, ($( $item:ident : $type:ty )*),  } => {

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

