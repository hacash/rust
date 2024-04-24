// Balance
StructFieldStruct!{ TxExist, 
	height:  BlockHeight
}




// TxPkg
#[derive(Clone)]
pub struct TxPackage {
	time: u64,
	hash: Hash,
	data: BytesW4,
    objc: Box<dyn Transaction>,
}

impl HashBodyPkg for TxPackage {

    fn time(&self) -> u64 { 
		self.time
	}
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
	
	pub fn new(blk: Box<dyn Transaction>) -> TxPackage {
		let dts = blk.serialize();
		TxPackage::new_with_data(blk, dts)
	}

	pub fn new_with_data(tx: Box<dyn Transaction>, data: Vec<u8>) -> TxPackage {
		TxPackage{
			time: curtimes(), // SystemTime::now()
			hash: tx.hash(),
			data: BytesW4::from_vec_u8(data),
			objc: tx,
		}
	}

}
