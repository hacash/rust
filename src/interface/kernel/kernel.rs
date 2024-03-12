
pub trait KernelRead {
    fn block<T>(&self, _: T) -> Option<impl BlockPkg>;
    fn store(&self);
}

pub trait Kernel : KernelRead {
    fn init(&self, _: &IniObj) -> Option<Error>;
    fn start(&self) -> Option<Error>;

    fn insert(&self, _: &impl BlockPkg) -> Option<Error>;
}


