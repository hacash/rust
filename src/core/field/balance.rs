// Satoshi

pub type Satoshi = Uint8;
impl Satoshi {}

// SatoshiOptional

StructFieldOptional!{ SatoshiOptional,
    satoshi, Satoshi
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


