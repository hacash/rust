
fn impl_genesis(this: &BlockMintChecker, state: &mut dyn State) -> RetErr {
    
	let addr1 = Address::form_readable("12vi7DEZjh6KrK5PVmmqSgvuJPCsZMmpfi").unwrap();
	let addr2 = Address::form_readable("1LsQLqkd8FQDh3R7ZhxC5fndNf92WfhM19").unwrap();
	let addr3 = Address::form_readable("1NUgKsTgM6vQ5nxFHGz1C4METaYTPgiihh").unwrap();
	let amt1 = Amount::new_small(1, 244);
	let amt2 = Amount::new_small(12, 244);
    let bls1 = Balance::hacash(amt1);
    let bls2 = Balance::hacash(amt2);
    let mut state = CoreState::wrap(state);
    state.set_balance(&addr1, &bls2);
    state.set_balance(&addr2, &bls1);
    state.set_balance(&addr3, &bls1);
    // ok
    Ok(())
} 
