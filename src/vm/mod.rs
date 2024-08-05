













































/*

pub struct HacashVM {
    store: Arc<dyn Store>,
}




impl VM for HacashVM {

    fn new(ini: &IniObj, sto: Arc<dyn Store>) -> HacashVM {
        HacashVM{
            store: sto,
        }
    }

    fn exec(&self, ctx: &dyn ExecContext, bst: &mut dyn State, con: &Vec<Box<dyn Action>>) -> RetErr {
        do_exec(ctx, bst, self.store.as_ref(), con)
    }

}



fn do_exec(ctx: &dyn ExecContext, bst: &mut dyn State, sto: &dyn Store, actlist: &Vec<Box<dyn Action>>) -> RetErr {
    
    for act in actlist {

        // ext action
        if act.kind() > 0 {
            // exec
            let res = act.execute(ctx, bst, sto);
            if let Some(abort_err) = res.abort() {
                return Err(abort_err.clone()) // abort error
            }
        }else{
            let kd = act.kind();
            return errf!("cannot exec action bykind {}", kd)
        }

    }
    
    // ok finish
    Ok(())
}


*/