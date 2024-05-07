#[macro_export]
macro_rules! fnUintFromToParseBytes {
    ($class:ident, $tarty:ident, $tsz:expr, $size:expr) => (

    concat_idents!(fn_to_1 = to_, $tarty {
    fn fn_to_1(&self) -> $tarty {
        let rv = bytes_to_uint(&self.bytes[..], $tsz, $size).unwrap();
        rv as $tarty
    }
    });

    concat_idents!(fn_parse_1 = parse_, $tarty {
    fn fn_parse_1(&mut self, val: $tarty) -> RetErr {
        let bts = bytes_from_uint(val as u64, $tsz, $size)?;
        self.bytes = bts.try_into().unwrap();
        Ok(())
    }
    });

    concat_idents!(fn_from_1 = from_, $tarty {
    fn fn_from_1(val: $tarty) -> Self where Self: Sized {
        let mut obj = <$class>::default();
        // obj.parse_uint(val as u64).unwrap();
        if let Err(e) = field_parse_uint(&mut obj, val as u64, $size) {
            panic!("{}", e)
        }
        obj
    }
    });

    )
}