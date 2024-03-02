

#[macro_export]
macro_rules! fnFieldCreate {
    ($class:ty) => (

fn create(buf: &[u8]) -> Result<($class, usize), Error> {
    let mut v = <$class>::new();
    let res = v.parse(buf, 0);
    match res {
        Ok(sk) => Ok((v, sk)),
        Err(e) => return Err(e),
    }
}

    )
}

