

pub fn create(buf: &[u8]) -> Ret<(Box<dyn Transaction>, usize)> {
    panic_never_call_this!()
}


pub fn clone(src: &dyn Transaction) -> Ret<Box<dyn Transaction>> {
    panic_never_call_this!()
}

