


// fork sub 
pub fn fork_sub_state(base: Arc<ChainState>) -> ChainState {
    ChainState{
        // memdb
        memk: MemoryDB::new(),
        disk: base.copy_ldb(),
        base: Arc::<ChainState>::downgrade(&base),
    }
}
