
// BlockPtr
StructFieldStruct!{ BlockPtr, 
	// ptr
	height : BlockHeight
	hash   : Hash
}


// BlockHead
StructFieldStruct!{ BlockHead, 
	// head
	version           : Uint1
	height            : BlockHeight
	timestamp         : Timestamp
	prevhash          : Hash
	mrklroot          : Hash
	transaction_count : Uint4
}


impl BlockHead {
	pub fn transaction_count(&self) -> &Uint4 {
		&self.transaction_count
	}
}


// BlockMeta
StructFieldStruct!{ BlockMeta, 
	// meta
	nonce         : Fixed4     // Mining random value
	difficulty    : Uint4       // Target difficulty value
	witness_stage : Fixed2     // Witness quantity level
}


// BlockHead&Meta
StructFieldStruct!{ BlockHeadMeta, 
	// head                   
	head : BlockHead
	meta : BlockMeta
}


impl BlockHeadMeta {
	pub fn transaction_count(&self) -> &Uint4 {
		self.head.transaction_count()
	}
}


pub type BlockIntro = BlockHeadMeta;

