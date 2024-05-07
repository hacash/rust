
// Balance
StructFieldStruct!{ Balance, 
	hacash:  Amount
	satoshi: SatoshiAuto
    diamond: DiamondNumberAuto
}

impl Balance {

	pub fn hacash(amt: Amount) -> Balance {
		Balance{
			hacash: amt,
			satoshi: SatoshiAuto::default(),
			diamond: DiamondNumberAuto::default(),
		}
	}

}
