
pub fn create(buf: impl AsRef<[u8]>) -> Box<dyn Block> {
    panic_never_call_this!()
}


pub fn create_pkg(buf: impl AsRef<[u8]>) -> Box<dyn BlockPkg> {
    panic_never_call_this!()
}
