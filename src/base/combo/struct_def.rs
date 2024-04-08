

#[macro_export]
macro_rules! StructFieldStruct {
    ($class: ident, $( $item: ident: $type: ty )+) => (



// #[derive(DebugDebug, Clone, PartialEq, Eq)]
pub struct $class {
    $(
        pub $item: $type
    ),+
}


impl Parse for $class {

    fn parse(&mut self, buf: &[u8], seek: usize) -> Ret<usize> {
        let mut sk: usize = seek;
        $(
            sk = self.$item.parse(buf, sk) ?;
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



// test
StructFieldStruct!{ Test83648354928437648,
    abc: Bool
    foo: Uint4
}