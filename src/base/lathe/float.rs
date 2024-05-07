
#[macro_export]
macro_rules! fnFloatFromToParseBytes {
    ($class:ident, $tarty:ident, $tsz:expr, $size:expr) => (

        concat_idents!(fn_to_1 = to_, $tarty {
        fn fn_to_1(&self) -> $tarty {
            let sz = $size;
            let tz = $tsz;
            if sz != tz {
                panic!("size error must be {}", tz)
            }
            <$tarty>::from_be_bytes(self.bytes[0..tz].try_into().unwrap())
        }
        });

        concat_idents!(fn_parse_1 = parse_, $tarty {
        fn fn_parse_1(&mut self, fv: $tarty) -> RetErr {
            let sz = $size;
            let tz = $tsz;
            if sz != tz {
                panic!("size error must be {}", tz)
            }
            let bts = fv.to_be_bytes();
            self.bytes = bts[0..tz].try_into().unwrap();
            Ok(())
        }
        });

        concat_idents!(fn_from_1 = from_, $tarty {
        fn fn_from_1(fv: $tarty) -> Self where Self: Sized {
            let mut obj = <$class>::default();
            // obj.parse_float(fv as f64).unwrap();
            field_parse_float(&mut obj, fv as f64, $size).unwrap();
            obj
        }
        });
    )
}

