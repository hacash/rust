

#[macro_export]
macro_rules! StructFieldStructSetParseSerializeSize {
    ($p_self:ident, $p_buf:ident, $p_seek:ident, $parse_code: block, $serialize_code: block, $size_code: block, $class: ident, $( $item: ident: $type: ty )+) => (

        

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub struct $class {
    $(
        pub $item: $type
    ),+
}


impl Parse for $class {

    fn parse(&mut $p_self, $p_buf: &[u8], $p_seek: usize) -> Ret<usize> {
        $parse_code;
        let mut sk: usize = $p_seek;
        $(
            sk = $p_self.$item.parse($p_buf, sk) ?;
        )+
        Ok(sk)
    }

}


impl Serialize for $class {

    fn serialize(&$p_self) -> Vec<u8> {
        $serialize_code;
        vec![
        $(
            $p_self.$item.serialize()
        ),+
        ].concat()
    }

    fn size(&$p_self) -> usize {
        $size_code;
        let mut size: usize = 0;
        $(
            size += $p_self.$item.size();
        )+
        size
    }

}

impl Field for $class {

    // must & create function
    fnFieldMustCreate!($class);
}

impl $class {

}



    )
}


#[macro_export]
macro_rules! StructFieldStructSetParse {
    ($p_self:ident, $p_buf:ident, $p_seek:ident, $parse_code: block, $class: ident, $( $item: ident: $type: ty )+) => (
        StructFieldStructSetParseSerializeSize!($p_self, $p_buf, $p_seek, $parse_code, {}, {}, $class, $( $item: $type )+ );
    )
}



#[macro_export]
macro_rules! StructFieldStruct {
    ($class: ident, $( $item: ident: $type: ty )+) => (
        StructFieldStructSetParse!(self, buf, seek, {}, $class, $( $item: $type )+ );
    )
}



// test
StructFieldStruct!{ Test83648354928437648,
    abc: Bool
    foo: Uint4
}