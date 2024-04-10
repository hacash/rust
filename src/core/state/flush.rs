

fn impl_flush_disk(this: &ChainState) {
    let mut batch = Writebatch::new();
    for (k, v) in this.memk.iter() {
        if let MemdbItem::Delete = v {
            batch.delete(k);
        } else if let MemdbItem::Value(v) = v {
            batch.put(k, &v);
        }
    }
    // flush to disk
    this.disk.write(&batch);
    // change mark, drop base
    // this.base = Weak::new().into();
}


/*
// roll copy
pub fn roll_copy_state(base: Arc<ChainState>, tar: &mut ChainState) -> Option<Error> {
    // tar.baseptr = None; // drop base ptr
    // tar.leveldb = base.leveldb.clone();

    None
}
*/