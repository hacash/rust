// Satoshi

pub type Satoshi = Uint8;
impl Satoshi {}

// Satoshi
pub type SatoshiAuto = AutoU64;
StructFieldOptional!{ SatoshiOptional,
    satoshi, Satoshi
}
impl SatoshiAuto {
	pub fn to_satoshi(&self) -> Satoshi {
		Satoshi::from( self.uint() )
	}
	pub fn from_satoshi(sat: &Satoshi) -> SatoshiAuto {
		SatoshiAuto::from( sat.uint() )
	}
}


// AddrHac
StructFieldStruct!{ AddrHac,
	address: Address
	amount : Amount
}

// HacAndSat
StructFieldStruct!{ HacSat, 
	amount : Amount
	satoshi: SatoshiOptional
}

// AddrHacSat
StructFieldStruct!{ AddrHacSat, 
	address: Address
	hacsat : HacSat
}


