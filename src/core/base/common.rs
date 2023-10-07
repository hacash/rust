

#[macro_export]
macro_rules! impl_entire_Field_trait_for_combine_class {
    ($class: ident, $( $child: ident: $cty: ty ),+) => (


impl_Serialize_trait_for_combine_class!{ $class, $( $child ),+ }
impl_Describe_trait_for_combine_class!{ $class, $( $child ),+ }

impl Field for $class {

    fn new() -> $class {
        $class {
        $(
            $child: <$cty>::new(),
        )+
        }
    }

    // create function
    fn_field_create_by_new_wrap_return!{ $class }
}

    
    )
}


///////////////////////



#[macro_export]
macro_rules! create_combine_class_and_impl_entire_Field_trait {
    ($class: ident, $( $child: ident: $cty: ty ),+) => (


pub struct $class {
$(
    pub $child: $cty,
)+
}


// Field + Serialize + Describe
impl_entire_Field_trait_for_combine_class! { $class, 
$(
    $child: $cty
),+
}



    )
}



