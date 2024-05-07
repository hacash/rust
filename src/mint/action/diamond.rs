
/**
 * Diamond Transfer
 */
 ActionDefine!{
    DiamondTransfer : 5, (
        diamond  : DiamondName
        to       : Address
    ),
    ACTLV_TOP, // level
    6 + 21, // gas
    (self, env, state, store), // params
    false, // burn 90
    [], // req sign
    ActExecRes::wrap(diamond_transfer(self, env, state, store))
}

fn diamond_transfer(this: &DiamondTransfer, env: &dyn ExecEnv, sta: &mut dyn State, sto: &dyn Store) -> RetErr {

    let from = env.main_address();
    // move
    let mut state = MintState::wrap(sta);
    hacd_move_one_diamond(&mut state, from, &this.to, &this.diamond)?;
    let mut list = DiamondNameListMax200::default();
    list.push(this.diamond);
    diamond_owned_move(&mut state, from, &this.to, &list)?;
    drop(state);
    // transfer
    let mut core_state = CoreState::wrap(sta);
    hacd_transfer(&mut core_state, from, &this.to, &DiamondNumber::from(1))
}




/**
 * Diamond Multiple From To Transfer
 */
 ActionDefine!{
    DiamondFromToTransfer : 6, (
        from           : Address 
        to             : Address 
        diamonds       : DiamondNameListMax200 
    ),
    ACTLV_TOP, // level
    21+21, // gas
    (self, env, state, store), // params
    false, // burn 90
    [self.from], // req sign
    {
        let mut res = ActExecRes::wrap(diamond_from_to_transfer(self, env, state, store));
        let addgas = self.diamonds.count().uint() as u32 * DiamondName::width() as u32;
        res.add_gas_use( addgas ); // gas = dia num * 6
        res
    }
}

fn diamond_from_to_transfer(this: &DiamondFromToTransfer, env: &dyn ExecEnv, sta: &mut dyn State, sto: &dyn Store) -> RetErr {

    // check
    let dianum = this.diamonds.check()?;
    //transfer
    let mut state = MintState::wrap(sta);
    for dianame in this.diamonds.list() {
        hacd_move_one_diamond(&mut state, &this.from, &this.to, &dianame)?; // move one
    }
    diamond_owned_move(&mut state, &this.from, &this.to, &this.diamonds)?;
    drop(state);
    // transfer
    let mut core_state = CoreState::wrap(sta);
    hacd_transfer(&mut core_state, &this.from, &this.to, &DiamondNumber::from(dianum as u32))
}



/**
 * Diamond Multipl Transfer
 */
 ActionDefine!{
    DiamondMultipleTransfer : 7, (
        to             : Address 
        diamonds       : DiamondNameListMax200 
    ),
    ACTLV_TOP, // level
    21, // gas
    (self, env, state, store), // params
    false, // burn 90
    [], // req sign
    {
        let mut res = ActExecRes::wrap(diamond_multiple_transfer(self, env, state, store));
        let addgas = self.diamonds.count().uint() as u32 * DiamondName::width() as u32;
        res.add_gas_use( addgas ); // gas = dia num * 6
        res
    }
}

fn diamond_multiple_transfer(this: &DiamondMultipleTransfer, env: &dyn ExecEnv, sta: &mut dyn State, sto: &dyn Store) -> RetErr {

    // check
    let dianum = this.diamonds.check()?;
    // from
    let from = env.main_address();
    //transfer
    let mut state = MintState::wrap(sta);
    for dianame in this.diamonds.list() {
        hacd_move_one_diamond(&mut state, from, &this.to, &dianame)?; // move one
    }
    diamond_owned_move(&mut state, from, &this.to, &this.diamonds)?;
    drop(state);
    // transfer
    let mut core_state = CoreState::wrap(sta);
    hacd_transfer(&mut core_state, from, &this.to, &DiamondNumber::from(dianum as u32))
}
