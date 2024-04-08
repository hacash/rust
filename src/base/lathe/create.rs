

#[macro_export]
macro_rules! fnFieldMustCreate {
    ($class:ty) => (

        fn must(buf: &[u8]) -> $class {
            let mut v = <$class>::new();
            let res = v.parse(buf, 0);
            match res {
                Ok(_) => v,
                Err(e) => panic!("{}", e),
            }
        }

        fn create(buf: &[u8]) -> Ret<($class, usize)> {
            let mut v = <$class>::new();
            let res = v.parse(buf, 0);
            match res {
                Ok(sk) => Ok((v, sk)),
                Err(e) => return Err(e),
            }
        }
                


    )
}

