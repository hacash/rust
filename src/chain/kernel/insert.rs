

impl Kernel for BlockChainKernel {

    fn insert(&mut self, blkpkg: &dyn BlockPkg) -> Option<Error> {
        impl_insert(self, blkpkg)
    }

}

// do insert
fn impl_insert(this: &mut BlockChainKernel, blkpkg: &dyn BlockPkg) -> Option<Error> {
    // lock
    this.isrlck.lock();
    // check height
    let block = blkpkg.objc();
    let isrhei = block.height().to_u64();
    // if isrhei <= this.sroot.height.to_u64();

    None
}