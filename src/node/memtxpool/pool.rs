

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
        let gid = self.deal_group_id(txp.as_ref(), gi)?;
        // do insert
        let mut grp = self.groups[gid].write().unwrap();
        grp.insert(txp)
    }

    fn delete(&self, hxs: &Vec<Hash>, gi: isize) {
        let gid = self.get_group_id(gi);
        // do delete
        let mut grp = self.groups[gid].write().unwrap();
        grp.delete(hxs)
    }

    fn clean(&self, gi: isize) {
        let gid = self.get_group_id(gi);
        // do clean
        let mut grp = self.groups[gid].write().unwrap();
        grp.clean()
    }


}



impl MemTxPool{ 
    // from group id
    fn find(&self, hx: &Hash, gi: isize) -> Option<Box<dyn TxPkg>> {
        let gid = self.get_group_id(gi);
        // do clean
        let grp = self.groups[gid].read().unwrap();
        match grp.find(hx) {
            Some((_, &ref tx)) => Some(tx.clone()),
            None => None,
        }
    }
    
}


