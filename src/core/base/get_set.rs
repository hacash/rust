

#[macro_export]
macro_rules! create_get_func_for_combine_class {
    ( $( $child_name: ident: $cty: ty )+) => (

    $(
        
    concat_idents!(get_func = get_, $child_name {
    fn get_func (&self) -> &$cty {
        &self.$child_name
    }
    });

    )+


    )
}

