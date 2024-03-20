
// Balance
StructFieldStruct!{ Balance, 
	hacash:  Amount
	satoshi: SatoshiOptional
    diamond: DiamondNumberOptional
}

impl Balance {

	pub fn hacash(amt: Amount) -> Balance {
		Balance{
			hacash: amt,
			satoshi: SatoshiOptional::new(),
			diamond: DiamondNumberOptional::new(),
		}
	}

}
