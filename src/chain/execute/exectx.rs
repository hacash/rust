
/**
 * do insert block crate new state
 * return new state
 */
pub fn do_exec_tx(pending_height: u64, vm: &dyn VM, bst: &mut dyn State, tx: &dyn TransactionRead) -> RetErr {
    // create exec env
    let env = ExecEnvObj::new(pending_height, tx);
    let exlist = tx.actions();
    // exec
    let exres = vm.exec(&env, bst, exlist) ? ;
    // ok
    Ok(())
}