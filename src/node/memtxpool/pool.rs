

pub struct MemTxPool {
    group_size: Vec<usize>,
    groups: Vec<RwLock<TxGroup>>,
}

impl MemTxPool {
    
    pub fn new(gs: Vec<usize>) -> MemTxPool {
        let mut grps = vec![];
        for sz in &gs {
            grps.push( RwLock::new( TxGroup::new(*sz)) );
        }
        MemTxPool {
            group_size: gs,
            groups: grps,
        }
    }

}

impl TxPool for MemTxPool {

    // insert to target group
    fn insert(&self, txp: Box<dyn TxPkg>, gi: isize) -> RetErr { 
        let gid = self.deal_group_id(txp.as_ref(), gi) ? ;
        // do insert
        let mut grp = self.groups[gid].write().unwrap();
        grp.insert(txp)
    }

    fn delete(&self, _: &Vec<Hash>, _: isize) {}
    fn clean(&self, _: isize) {}

    // from group id
    fn find(&self, _: &Hash, _: isize) -> Option<Box<dyn TxPkg>> { None }
    
}
