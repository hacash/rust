// Balance
StructFieldStruct!{ TxExist, 
	height:  BlockHeight
}




// TxPkg
pub struct TxPackage {
	pub hash: Hash,
	pub data: BytesW4,
    pub objc: Box<dyn Tx>,
}

impl HashBodyPkg for TxPackage {

    fn hash(&self) -> &Hash { 
		&self.hash
	}
    fn body(&self) -> &BytesW4 {
		&self.data
	}
}

impl BlockPkg for TxPackage {

    fn objc(&self) -> &Box<dyn Tx> { 
		&self.objc
	}
	
}

impl TxPackage {
	pub fn new(tx: Box<dyn Tx>) -> TxPackage {
		TxPackage{
			hash: tx.hash(),
			data: BytesW4::from_vec_u8(tx.serialize()),
			objc: tx,
		}
	}

}
