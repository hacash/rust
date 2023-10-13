

#[macro_export]
macro_rules! field_serialize_items_concat {
    ($( $child: expr ),+) => (
        vec![
        $(
            $child.serialize(),
        )*
        ].concat()
    )
}


#[macro_export]
macro_rules! impl_Serialize_trait_for_combine_class {

    ($class: ident, $( $child: ident ),+) => (

        impl Serialize for $class {

            fn serialize(&self) -> Vec<u8> {
                field_serialize_items_concat!{ $( self.$child ),+ }
            }

            fn parse(&mut self, buf: &Vec<u8>, seek: usize) -> Result<usize, Error> {
                let mut sk: usize = seek;
                $(
                    sk = self.$child.parse(buf, sk) ? ;
                )*
                Ok(sk)
            }

            fn size(&self) -> usize {
                let mut size: usize = 0;
                $(
                    size += self.$child.size();
                )*
                size
            }

        }



    )
}

