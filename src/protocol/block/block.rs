

// Trs list
StructFieldDynVec!{
    DynVecTransaction, 
    Uint4, Transaction, transaction::create
}



// BlockV1
StructFieldStruct!{ BlockV1, 
    // head meta
	intro : BlockIntro
	// trs body
	transactions : DynVecTransaction
}

impl BlockRead for BlockV1 {
    
}

impl Block for BlockV1 {
    
}


impl BlockV1 {
    pub fn new() -> BlockV1 {
        let mut blk = <BlockV1 as Field>::new();
        blk.intro.head.version = Uint1::from_u8(1);
        blk 
    }
}

