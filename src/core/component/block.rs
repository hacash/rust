

// BlockPkg


// BlockPkg
#[derive(Clone)]
pub struct BlockPackage {
	time: u64,
	hash: Hash,
	data: BytesW4,
    objc: Box<dyn Block>,
}

impl HashBodyPkg for BlockPackage {

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

impl BlockPkg for BlockPackage {

    fn objc(&self) -> &Box<dyn Block> { 
		&self.objc
	}
	
}

impl BlockPackage {
	pub fn new(blk: Box<dyn Block>) -> BlockPackage {
		let dts = blk.serialize();
		BlockPackage::new_with_data(blk, dts)
	}

	pub fn new_with_data(blk: Box<dyn Block>, data: Vec<u8>) -> BlockPackage {
		BlockPackage{
			time: curtimes(), // SystemTime::now()
			hash: blk.hash(),
			data: BytesW4::from_vec_u8(data),
			objc: blk,
		}
	}

}
