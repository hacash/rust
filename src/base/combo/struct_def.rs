

#[macro_export]
macro_rules! StructFieldStructSetParse {
    ($p_self:ident, $p_buf:ident, $p_seek:ident, $parse_code: block, $class: ident, $( $item: ident: $type: ty )+) => (



#[derive(Debug, Clone, PartialEq, Eq)]
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

    fn serialize(&self) -> Vec<u8> {
        vec![
        $(
            self.$item.serialize()
        ),+
        ].concat()
    }

    fn size(&self) -> usize {
        let mut size: usize = 0;
        $(
            size += self.$item.size();
        )+
        size
    }

}

impl Field for $class {

    fn new() -> $class {
        $class{
            $(
                $item: <$type>::new()
            ),+
        }
    }

    // must & create function
    fnFieldMustCreate!($class);
}

impl $class {

}



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