

#[macro_export]
macro_rules! fn_field_create_by_new_wrap_return {
    ($class:ty) => (

fn create(buf: &Vec<u8>, seek: usize) -> Result<($class, usize), Error> {
    let mut v = <$class>::new();
    let res = v.parse(buf, seek);
    match res {
        Ok(sk) => Ok((v, sk)),
        Err(e) => return Err(e),
    }
}

    )
}

