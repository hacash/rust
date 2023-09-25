

macro_rules! create_combine_field_struct_and_impl{
    ($tip: expr, $class: ident, $( $name: ident: $type: ty, )+) => (



#[derive(Clone)]
pub struct $class {
    $(
        pub $name: $type
    ),+
}


impl Serialize for $class {

    fn serialize(&self) -> Vec<u8> {
        vec![
        $(
            self.$name.serialize(),
        )*
        ].concat()
    }

    fn parse(&mut self, buf: &Vec<u8>, seek: usize) -> Result<usize, Error> {
        let mut sk: usize = seek;
        $(
            sk = self.$name.parse(buf, sk) ? ;
        )*
        Ok(sk)
    }

    fn size(&self) -> usize {
        let mut size: usize = 0;
        $(
            size += self.$name.size();
        )*
        size
    }

}


impl Describe for $class {

    fn describe(&self) -> String {
        "".to_string()
    }

    fn to_json(&self) -> String {
        "".to_string()
    }

    fn from_json(&mut self, _: &String) -> Option<Error> {
        None
    }

}



impl Field for $class {

    // create function
    pub_fn_field_create_by_new_wrap_return!($class);

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
create_combine_field_struct_and_impl!("Test", Test8364835492648,
    abc: Bool,
    foo: NumUInt4,
);