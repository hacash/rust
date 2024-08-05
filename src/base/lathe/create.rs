

#[macro_export]
macro_rules! fnFieldMustCreate {
    ($class:ty) => (

        fn new() -> $class {
            <$class>::default()
        }

        fn must(buf: &[u8]) -> $class {
            let mut v = <$class>::default();
            let res = v.parse(buf, 0);
            match res {
                Ok(_) => v,
                Err(e) => panic!("{}", e),
            }
        }

        fn build(buf: &[u8]) -> Ret<$class> {
            let mut v = <$class>::default();
            let res = v.parse(buf, 0);
            match res {
                Ok(sk) => Ok(v),
                Err(e) => return Err(e),
            }
        }

        fn create(buf: &[u8]) -> Ret<($class, usize)> {
            let mut v = <$class>::default();
            let res = v.parse(buf, 0);
            match res {
                Ok(sk) => Ok((v, sk)),
                Err(e) => return Err(e),
            }
        }
                


    )
}

