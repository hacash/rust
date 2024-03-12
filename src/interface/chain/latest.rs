

pub trait StoreLatest {
    fn height(&self) -> &BlockHeight;
}

