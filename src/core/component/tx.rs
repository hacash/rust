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

    fn fee_purity(&self) -> u64 {
		let txsz = self.data.length() as u64;
		let feeshuo = self.objc.fee_got().to_shuo_unsafe() as u64;
		feeshuo / txsz
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
			data: BytesW4::from_vec(data),
			objc: tx,
		}
	}

}
