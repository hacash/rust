

// Trs list
StructFieldDynList!{
    DynListTransaction, 
    Uint4, Transaction, transaction::create
}



// BlockV1
StructFieldStruct!{ BlockV1, 
    // head meta
	intro : BlockIntro
	// trs body
	transactions : DynListTransaction
}

impl BlockRead for BlockV1 {
    
}

impl Block for BlockV1 {
    
}

