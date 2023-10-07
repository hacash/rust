



#[macro_export]
macro_rules! create_combine_field_struct_and_impl{
    ($tip: expr, $class: ident, $( $name: ident: $type: ty )+) => (



#[derive(Debug, Clone, PartialEq, Eq)]
pub struct $class {
    $(
        pub $name: $type
    ),+
}


impl_Serialize_trait_for_combine_class!( $class, $( $name ),+ );

impl_Describe_trait_for_combine_class!( $class, $( $name ),+ );


impl Field for $class {

    // create function
    fn_field_create_by_new_wrap_return!($class);

    fn new() -> $class {
        $class{
            $(
                $name: <$type>::new()
            ),+
        }
    }

}

impl $class {

}



    )
}



// test
create_combine_field_struct_and_impl!{ "Test", Test8364835492648,
    abc: Bool
    foo: Uint4
}