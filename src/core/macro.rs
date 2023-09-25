
#[macro_export] 
macro_rules! pub_fn_field_create_by_new_wrap_return{
    ($name:ty) => (
        fn create(buf: &Vec<u8>, seek: usize) -> Result<($name, usize), Error> {
            let mut v = <$name>::new();
            let res = v.parse(buf, seek);
            match res {
                Ok(sk) => Ok((v, sk)),
                Err(e) => return Err(e),
            }
        }
    )
}
