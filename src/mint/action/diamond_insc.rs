
/**
 * Diamond Inscription
 */
 ActionDefine!{
    DiamondInscription : 32, (
        diamonds          : DiamondNameListMax200
        protocol_cost     : Amount   // HAC amount for burning
        engraved_type     : Uint1 //  0:String  1:CompressedDict  51:MD5  52:SHA256 ....
        engraved_content  : BytesW1
    ),
    ACTLV_TOP_ONLY, // level
    11 + 1, // gas
    (self, env, state, store), // params
    false, // burn 90
    [], // req sign
    {
        let mut res = ActExecRes::wrap(diamond_inscription(self, env, state, store));
        let moregasuse = 0u32;
        res.add_gas_use(moregasuse);
        res
    }
}

fn diamond_inscription(this: &DiamondInscription, env: &dyn ExecEnv, sta: &mut dyn State, sto: &dyn Store) -> RetErr {

    let main_addr = env.main_address();
    let pcost = &this.protocol_cost;

    // check
    this.diamonds.check() ? ;
	if pcost.size() > 4 {
		return errf!("protocol cost amount size cannot over 4 bytes")
	}
	// check insc size and visible
    let insc_len = this.engraved_content.length();
    if insc_len == 0 {
		return errf!("engraved content cannot be empty")
    }
    if insc_len > 64 {
		return errf!("engraved content size cannot over 64 bytes")
    }
    let insc_ty = this.engraved_type.uint();
    if insc_ty <= 50 {
        if ! check_readable_string(this.engraved_content.as_ref()) {
            return errf!("engraved content must readable string")
        }
    }

    // cost
    let mut ttcost = Amount::new();
    let pdhei = env.pending_height();

    // do
    let mut state = MintState::wrap(sta);
    let store = MintStoreDisk::wrap(sto);
    for dia in this.diamonds.list() {
        let cc = engraved_one_diamond(pdhei, &mut state, &store, main_addr, &dia, &this.engraved_content) ? ;
        ttcost = ttcost.add(&cc) ? ;
    }

	// check cost
	if pcost.less_than(&ttcost) {
		return errf!("diamond inscription cost error need {} but got {}", ttcost.to_fin_string(), pcost.to_fin_string())
	}

    // change count
    let mut ttcount = state.total_count();
    ttcount.diamond_engraved += this.diamonds.count().uint() as u64;
    ttcount.diamond_insc_burn_zhu += pcost.to_zhu_unsafe() as u64;
    state.set_total_count(&ttcount);

    drop(state);

    let mut core_state = CoreState::wrap(sta);
	// sub main addr balance
	if pcost.is_positive() {
        hac_sub(&mut core_state, main_addr, &pcost) ? ;
	}

    // finish
    Ok(())

}