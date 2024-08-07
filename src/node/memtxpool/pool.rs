

pub struct MemTxPool {
    group_size: Vec<usize>,
    groups: Vec<Mutex<TxGroup>>,
}

impl MemTxPool {
    
    pub fn new(gs: Vec<usize>) -> MemTxPool {
        let mut grps = vec![];
        for sz in &gs {
            grps.push( Mutex::new( TxGroup::new(*sz)) );
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
        let mut grp = self.groups[gid].lock().unwrap();
        grp.insert(txp)
    }

    fn delete(&self, hxs: &Vec<Hash>, gi: isize) {
        let gid = self.get_group_id(gi);
        // do delete
        let mut grp = self.groups[gid].lock().unwrap();
        grp.delete(hxs)
    }

    fn clean(&self, gi: isize) {
        let gid = self.get_group_id(gi);
        // do clean
        let mut grp = self.groups[gid].lock().unwrap();
        grp.clean()
    }

    // from group id
    fn find(&self, hx: &Hash, gi: isize) -> Option<Box<dyn TxPkg>> {
        let gid = self.get_group_id(gi);
        // do clean
        let grp = self.groups[gid].lock().unwrap();
        match grp.find(hx) {
            Some((_, &ref tx)) => Some(tx.clone()),
            None => None,
        }
    }
    
    // find
    fn find_all(&self, hx: &Hash) -> Option<Box<dyn TxPkg>> {
        for gi in 0..self.groups.len() as isize {
            if let Some(tx) = self.find(hx, gi) {
                return Some(tx) // ok find
            }
        }
        // not find
        None
    }

}

