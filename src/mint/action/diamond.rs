
/**
 * Diamond Transfer
 */
 ActionDefine!{
    DiamondTransfer : 5, (
        diamond  : DiamondName
        to       : AddrOrPtr
    ),
    ACTLV_MAIN, // level
    6 + 21, // gas
    (self, ctx, state, store, gas), // params
    false, // burn 90
    [], // req sign
    diamond_transfer(self, ctx, state, store)
}

fn diamond_transfer(this: &DiamondTransfer, ctx: &mut dyn ExecContext, sta: &mut dyn State, sto: &dyn Store) -> Ret<Vec<u8>> {

    let from = ctx.main_address().clone();
    // move
    let mut state = MintState::wrap(sta);
    let to = this.to.real(ctx.addr_list())?;
    hacd_move_one_diamond(&mut state, &from, &to, &this.diamond)?;
    let mut list = DiamondNameListMax200::default();
    list.push(this.diamond);
    diamond_owned_move(&mut state, &from, &to, &list)?;
    drop(state);
    // transfer
    let mut core_state = CoreState::wrap(sta);
    hacd_transfer(ctx, &mut core_state, &from, &to, &DiamondNumber::from(1), &list)
}




/**
 * Diamond Multiple From To Transfer
 */
 ActionDefine!{
    DiamondFromToTransfer : 6, (
        from           : AddrOrPtr 
        to             : AddrOrPtr 
        diamonds       : DiamondNameListMax200 
    ),
    ACTLV_MAIN, // level
    21+21, // gas
    (self, ctx, state, store, gas), // params
    false, // burn 90
    [self.from], // req sign
    {
        // gas = dia num * 6
        gas += self.diamonds.count().uint() as i64 * DiamondName::width() as i64;
        diamond_from_to_transfer(self, ctx, state, store)
    }
}

fn diamond_from_to_transfer(this: &DiamondFromToTransfer, ctx: &mut dyn ExecContext, sta: &mut dyn State, sto: &dyn Store) -> Ret<Vec<u8>> {

    // check
    let dianum = this.diamonds.check()?;
    //transfer
    let mut state = MintState::wrap(sta);
    let from = this.from.real(ctx.addr_list())?;
    let to = this.to.real(ctx.addr_list())?;
    for dianame in this.diamonds.list() {
        hacd_move_one_diamond(&mut state, &from, &to, &dianame)?; // move one
    }
    diamond_owned_move(&mut state, &from, &to, &this.diamonds)?;
    drop(state);
    // transfer
    let mut core_state = CoreState::wrap(sta);
    hacd_transfer(ctx, &mut core_state, &from, &to, &DiamondNumber::from(dianum as u32), &this.diamonds)
}



/**
 * Diamond Multipl Transfer
 */
 ActionDefine!{
    DiamondMultipleTransfer : 7, (
        to             : AddrOrPtr
        diamonds       : DiamondNameListMax200 
    ),
    ACTLV_MAIN, // level
    21, // gas
    (self, ctx, state, store, gas), // params
    false, // burn 90
    [], // req sign
    {
        // gas = dia num * 6
        gas += self.diamonds.count().uint() as i64 * DiamondName::width() as i64;
        diamond_multiple_transfer(self, ctx, state, store)
    }
}

fn diamond_multiple_transfer(this: &DiamondMultipleTransfer, ctx: &mut dyn ExecContext, sta: &mut dyn State, sto: &dyn Store) -> Ret<Vec<u8>> {

    // check
    let dianum = this.diamonds.check()?;
    // from
    let from = ctx.main_address().clone();
    //transfer
    let mut state = MintState::wrap(sta);
    let to = this.to.real(ctx.addr_list())?;
    for dianame in this.diamonds.list() {
        hacd_move_one_diamond(&mut state, &from, &to, &dianame)?; // move one
    }
    diamond_owned_move(&mut state, &from, &to, &this.diamonds)?;
    drop(state);
    // transfer
    let mut core_state = CoreState::wrap(sta);
    hacd_transfer(ctx, &mut core_state, &from, &to, &DiamondNumber::from(dianum as u32), &this.diamonds)
}
