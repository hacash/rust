

pub fn do_exec(env: &dyn ExecEnv, bst: &mut dyn State, sto: &dyn Store, actlist: &Vec<Box<dyn VMAction>>) -> RetErr {
    
    for act in actlist {

        // ext action
        if act.kind() > 0 {
            let extact = act.as_ext();
            // exec
            let res = extact.execute(env, bst, sto);
            if let Some(abort_err) = res.abort() {
                return Err(abort_err.clone()) // abort error
            }
        }else{
            let (cd, kd) = (act.code(), act.kind());
            return errf!("cannot exec action by code {} or kind {}", cd, kd)
        }

    }
    
    // ok finish
    Ok(())
}