



macro_rules! define_action_kind_class_parse_func {
    ( $kindv:expr, $class:ident, { $( $k: ident: $ty:ty )+ }) => (



// kind define
concat_idents!(act_kind_value = ACTION_KIND_, $kindv {
pub const act_kind_value: u16 = $kindv;
});



concat_idents!(act_parse_func = action_parse_func_, $kindv {
fn act_parse_func(buf: &Vec<u8>, sk: usize) -> Result<(Box<dyn Action>, usize), Error> {
    let (act, mvsk) = <$class>::create(buf, sk) ? ;
    Ok((Box::new(act), mvsk))
}
});


// func
concat_idents!(new_act_func = new_, $class {
pub fn new_act_func() -> $class {
    $class {
        kind: Uint2::from_uint($kindv),
        $(
            $k: <$ty>::new(),
        )*
    }
}
});
    


pub struct $class {
    pub kind: Uint2,
    $(
        pub $k: $ty,
    )*
}

impl_Serialize_trait_for_combine_class!($class, kind, $( $k ),+);

impl_Describe_trait_for_combine_class!($class, kind, $( $k ),+);

/* 
impl Serialize for act_kind_class_name {

    fn serialize(&self) -> Vec<u8> {
        vec![
            self.kind.serialize(),
        $(
            self.$k.serialize(),
        )*
        ].concat()
    }

    fn parse(&mut self, buf: &Vec<u8>, seek: usize) -> Result<usize, Error> {
        let mut sk: usize = seek;
        sk = self.kind.parse(buf, sk) ? ;
        $(
            sk = self.$k.parse(buf, sk) ? ;
        )*
        Ok(sk)
    }

    fn size(&self) -> usize {
        let mut size: usize = 0;
        size += self.kind.size();
        $(
            size += self.$k.size();
        )*
        size
    }

}


impl Describe for $class {

    fn describe(&self) -> String {
        "".to_string()
    }

    fn to_json(&self, cnf: &FieldJsonConfig) -> String {
        "".to_string()
    }

    fn from_json(&mut self, _: &String) -> Option<Error> {
        None
    }

}
*/

impl Field for $class {

    fn new() -> $class {
        $class{
            kind: Uint2::new(),
            $(
                $k: <$ty>::new(),
            )*
        }

    }

    // create function
    fn_field_create_by_new_wrap_return!($class);
} 

impl Action for $class {

    fn get_kind(&self) -> u16 {
        self.kind.to_u16()
    }

} 


impl $class {
    
    const fn kind() -> u16 {
        $kindv
    }

}






    )
}


//

macro_rules! actions_parse_func_and_include {
    ( $( $kindv: expr, )+ ) => (

// include mod files
$(

include!{stringify!($kindv.act.rs)}

)+

// parse func

pub fn create(buf: &Vec<u8>, seek: usize) -> Result<(Box<dyn Action>, usize), Error> {
    let (kindv, sk) = create_field_or_error!("action.create", Uint2, buf, seek);
    let kdv = kindv.to_u16();
    match kdv {
    $(
        concat_idents!(act_kind_value = ACTION_KIND_, $kindv { act_kind_value }) 
        => 
        concat_idents!(act_parse_func = action_parse_func_, $kindv { act_parse_func(buf, sk) }),
    )+
    _ => Err(format!("Action kind <{}> not find.", kdv))
    }
}



    )
}

