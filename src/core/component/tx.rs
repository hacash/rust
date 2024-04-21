// Balance
StructFieldStruct!{ TxExist, 
	height:  BlockHeight
}




// TxPkg
#[derive(Clone)]
pub struct TxPackage {
	pub hash: Hash,
	pub data: BytesW4,
    pub objc: Box<dyn Transaction>,
}

impl HashBodyPkg for TxPackage {

    fn hash(&self) -> &Hash { 
		&self.hash
	}
    fn body(&self) -> &BytesW4 {
		&self.data
	}
}

impl TxPkg for TxPackage {

    fn objc(&self) -> &Box<dyn Transaction> { 
		&self.objc
	}
	
}

impl TxPackage {
	pub fn new(tx: Box<dyn Transaction>) -> TxPackage {
		TxPackage{
			hash: tx.hash(),
			data: BytesW4::from_vec_u8(tx.serialize()),
			objc: tx,
		}
	}

}
