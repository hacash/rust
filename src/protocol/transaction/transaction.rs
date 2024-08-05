// Action list
StructFieldDynList!{
    DynListAction, 
    Uint2, Action, action::create
}


// TransactionType1
DefineCommonTransaction!{
    TX_TYPE_1_DEPRECATED, TransactionType1
}


// TransactionType2
DefineCommonTransaction!{
    TX_TYPE_2, TransactionType2
}


// TransactionType3
DefineCommonTransaction!{
    TX_TYPE_3, TransactionType3
}



pubFnTransactionsTypeDefineCreate!{

    TX_TYPE_0_COINBASE   , 0u8, TransactionCoinbase
    TX_TYPE_1_DEPRECATED , 1u8, TransactionType1
    TX_TYPE_2            , 2u8, TransactionType2
    TX_TYPE_3            , 3u8, TransactionType3

}


pub fn create_pkg(bytes: BytesW4) -> Ret<Box<dyn TxPkg>> {
    let buf = bytes.as_ref();
    let (txobj, _) = create(buf)?;
    let hash = txobj.hash();
    Ok(Box::new(TxPackage::new_with_data(txobj, bytes.into_vec())))
}




