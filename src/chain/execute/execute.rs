
/**
*
*/
pub fn exec_tx_actions(is_fast_sync: bool, 
    pending_height: u64, pending_hash: Hash, 
    bst: &mut dyn State, sto: &dyn Store, tx: &dyn TransactionRead,
) -> RetErr {

    let (a,b,c,d,e,f) = (is_fast_sync, pending_height, pending_hash, bst, sto, tx);
    let no_need_vm = tx.ty() < transaction::TX_TYPE_3 || tx.gas_max() <= 0;

    match no_need_vm {
        true  => exec_tx_actions_normal(a,b,c,d,e,f),
        false => exec_tx_actions_withvm(a,b,c,d,e,f),
    }
}


////////////////////////////////////////////////////////////////////


fn exec_tx_actions_normal(is_fast_sync: bool, 
    pending_height: u64, pending_hash: Hash, 
    bst: &mut dyn State, sto: &dyn Store, tx: &dyn TransactionRead,
) -> RetErr {

    // context & env
    let mut ctx = ExecEnvObj::new(pending_height, tx);
    // ptr
    ctx.pdhash = pending_hash;
    ctx.fastsync = is_fast_sync;
    let ctxptr: *mut ExecEnvObj = &mut ctx;

    // create env
    let mut extcaller = ExecCaller::new(ctxptr, bst, sto);

    // exec not vm
    let exlist = tx.actions();
    let call_depth = -1i8;
    for act in exlist {
        extcaller.exec(act.as_ref(), call_depth)?;
        // ignore return value
    }
    
    Ok(())
}



fn exec_tx_actions_withvm(is_fast_sync: bool, 
    pending_height: u64, pending_hash: Hash, 
    bst: &mut dyn State, sto: &dyn Store, tx: &dyn TransactionRead,
) -> RetErr {
    errf!("cannot exec tx with vm")
}

