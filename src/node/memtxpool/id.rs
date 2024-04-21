

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
