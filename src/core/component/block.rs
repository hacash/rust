

// BlockPkg


// BlockPkg
pub struct BlockPackage {
	pub hash: Hash,
	pub data: BytesW4,
    pub objc: Box<dyn Block>,
}

impl HashBodyPkg for BlockPackage {

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
		BlockPackage{
			hash: blk.hash(),
			data: BytesW4::from_vec_u8(blk.serialize()),
			objc: blk,
		}
	}

}
