
fn scan_group_rng_by_feep(txpkgs: &Vec<Box<dyn TxPkg>>, feep: u32, wsz: (usize, usize)) -> (usize, usize) {
    let mut rxl = wsz.0;
    let mut rxr = wsz.1;
    // scan rng
    loop {
        let rng = rxr-rxl;
        if rng <= 10 {
            break // end
        }
        let fct = rxl + rng/2;
        let ct = &txpkgs[fct];
        let cfp = ct.fee_purity();
        if feep > cfp {
            rxl = fct; // in right
        } else if feep < cfp {
            rxr = fct; // in left
        }else {
            // feep == cfp
            break // end
        }
    }
    // ok
    (rxl, rxr)
}


impl MemTxPool {


    fn get_group_id(&self, wgi: isize) -> usize {
        let mut gi = wgi;
        let gimx = self.groups.len() as isize - 1;
        if gi < 0 || gi > gimx {
            return 0
        }
        gi as usize
    }


    fn deal_group_id(&self, txp: &dyn TxPkg, wgi: isize) -> Ret<usize> {
        let mut gi = wgi;
        let gimx = self.groups.len() as isize - 1;
        if gi < 0 || gi > gimx {
            gi = txp.tx_pool_group() as isize;
        }
        if gi < 0 || gi > gimx {
            return errf!("cannot find group id {}", gi)
        }
        Ok(gi as usize)
    }


}
