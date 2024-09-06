

pub trait TxPool: Send + Sync {
    fn count_at(&self, _: usize) -> Ret<usize> { Ok(0) }
    fn iter_at(&self, _: &mut dyn FnMut(&Box<dyn TxPkg>)->bool, _: usize) -> RetErr { Ok(()) }
    fn insert_at(&self, _: Box<dyn TxPkg>, _: usize) -> RetErr { Ok(()) } // from group id
    fn delete_at(&self, _: &Vec<Hash>, _: usize) -> RetErr { Ok(()) } // from group id
    fn find_at(&self, _: &Hash, _: usize) -> Option<Box<dyn TxPkg>> { None } // from group id
    fn clear_at(&self, _: usize) -> RetErr { Ok(()) } // by group id
    fn drain_filter_at(&self, _: &dyn Fn(&Box<dyn TxPkg>)->bool, _: usize) 
        -> RetErr { Ok(()) }

    fn find(&self, hx: &Hash) -> Option<Box<dyn TxPkg>> { None }
    fn insert(&self, _: Box<dyn TxPkg>) -> RetErr { Ok(()) }
    fn drain(&self, _: &Vec<Hash>) -> Ret<Vec<Box<dyn TxPkg>>> { Ok(vec![]) }

    fn print(&self) -> String { s!("") }
}