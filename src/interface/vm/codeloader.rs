
pub trait FnCodePkg {
    fn is_cache(&self) -> bool;
    fn kind(&self) -> u8;
    fn body(&self) -> &[u8];
}


pub trait CodeLoader {

    fn load(&self, contract: Address, fnsign: Fixed4, cached: bool) -> Result<Box<dyn FnCodePkg>, Error>;



}