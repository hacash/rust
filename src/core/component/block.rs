

// BlockPkg


// BlockPkg
pub struct BlockPackage {
	hash: Hash,
	data: BytesW4,
    objc: Box<dyn Block>,
}

impl HashBodyPkg for BlockPackage {

}

impl BlockPkg for BlockPackage {
	
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
