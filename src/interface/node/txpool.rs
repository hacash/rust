

pub trait TxPool {
    fn insert(&self, _: Box<dyn TxPkg>, _: isize) -> RetErr { Ok(()) } // from group id
    fn delete(&self, _: &Vec<Hash>, _: isize) {} // from group id
    fn clean(&self, _: isize) {} // by group id
    
    fn find(&self, _: &Hash, _: isize) -> Option<Box<dyn TxPkg>> { None } // from group id

}