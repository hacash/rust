
macro_rules! pub_struct_field_define_for_common{
    ($class: ident, $( $value: ident, $value_type: ty,)+) => (

// 
#[derive(Clone)]
pub struct $class {
    $(
        pub $value: $value_type,
    )+
}

impl $class {

    // create function
    pub_fn_field_create_by_new_wrap_return!($class);

}


// impl Field for Sign
impl_Field_trait_for_common!(0, $class, 
    $(
        $value, $value_type,
    )+
);


    )
}
