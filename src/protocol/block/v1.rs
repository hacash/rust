

// Trs list
StructFieldDynVec!{
    DynVecTransaction, 
    Uint4, Transaction, transaction::create
}



// BlockV1
StructFieldStructSetParse!{ self, buf, seek, {
    return {
        // intro
        let mut intro = BlockIntro::new();
        let seek = intro.parse(buf, seek)?;
        let trslen = intro.head.transaction_count.to_u64();
        self.intro = intro;
        // body
        self.transactions.set_count(trslen);
        self.transactions.parse(buf, seek)
    }
},
    BlockV1, 
    // head meta
	intro : BlockIntro
	// trs body
	transactions : DynVecTransaction
}


/********************/


impl BlockRead for BlockV1 {

    fn hash(&self) -> Hash {
        let intro = self.intro.serialize();
        let hx = x16rs::block_hash(self.height().to_u64(), intro);
        Hash::must(&hx[..])
    }

    fn height(&self) -> &BlockHeight {
        &self.intro.head.height
    }

    fn timestamp(&self) -> &Timestamp {
        &self.intro.head.timestamp
    }

    fn prevhash(&self) -> &Hash {
        &self.intro.head.prevhash
    }

    fn mrklroot(&self) -> &Hash {
        &self.intro.head.mrklroot
    }
    
    fn transaction_count(&self) -> &Uint4 {
        self.intro.transaction_count()
    }

    fn transaction_hash_list(&self, hash_with_fee: bool) -> Vec<Hash> {
        let mut list = vec![];
        // println!("self.transactions.list: {}", self.transactions.list().len());
        for t in self.transactions.list() {
            if hash_with_fee {
                list.push(t.hash_with_fee())
            }else{
                list.push(t.hash())
            }
        }
        list
    }

    fn transactions(&self) -> &Vec<Box<dyn Transaction>> {
        self.transactions.list()
    }


}


/********************/


impl Block for BlockV1 {

    fn update_mrklroot(&mut self) {
        let hash_with_fee = true;
        let hxlist = self.transaction_hash_list(hash_with_fee);
        let mrkl = calculate_mrklroot(&hxlist);
        self.set_mrklroot(mrkl);
    }

    fn set_mrklroot(&mut self, mkrt: Hash) {
        self.intro.head.mrklroot = mkrt;
    }

    
}



/********************/


impl BlockV1 {
    pub fn new() -> BlockV1 {
        let mut blk = <BlockV1 as Field>::new();
        blk.intro.head.version = Uint1::from(BLOCK_VERSION_1);
        blk 
    }
}

