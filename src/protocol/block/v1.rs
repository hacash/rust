

// Trs list
StructFieldDynVec!{
    DynVecTransaction, 
    Uint4, Transaction, transaction::create
}



// BlockV1
StructFieldStructSetParse!{ self, buf, seek, {
    return {
        // intro
        let mut intro = BlockIntro::default();
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

macro_rules! block_intro_fn_mount{
    ($fname:ident, $rty:ty) => (
        fn $fname(&self) -> &$rty {
            self.intro.$fname()
        }
    )
}

impl BlockRead for BlockV1 {

    fn hash(&self) -> Hash {
        self.intro.hash()
    }

    block_intro_fn_mount!{version, Uint1}
    block_intro_fn_mount!{height, BlockHeight}
    block_intro_fn_mount!{timestamp, Timestamp}
    block_intro_fn_mount!{difficulty, Uint4}
    block_intro_fn_mount!{nonce, Uint4}
    block_intro_fn_mount!{prevhash, Hash}
    block_intro_fn_mount!{mrklroot, Hash}
    block_intro_fn_mount!{transaction_count, Uint4}

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

    fn as_read(&self) -> &dyn BlockRead { 
        self
    }

    fn update_mrklroot(&mut self) {
        let hash_with_fee = true;
        let hxlist = self.transaction_hash_list(hash_with_fee);
        let mrkl = calculate_mrklroot(&hxlist);
        self.set_mrklroot(mrkl);
    }

    fn set_mrklroot(&mut self, mkrt: Hash) {
        self.intro.head.mrklroot = mkrt;
    }

	fn set_nonce(&mut self, nonce: Uint4) {
        self.intro.meta.nonce = nonce;
	}

    fn replace_transaction(&mut self, i: usize, v: Box<dyn Transaction>) -> RetErr {
        let tl = self.transactions.vlist.len();
        if i >= tl {
            return errf!("transaction index overflow")
        }
        self.transactions.vlist[i] = v;
        Ok(())
    }

    fn push_transaction(&mut self, tx: Box<dyn Transaction>) -> RetErr {
        let ct = &mut self.intro.head.transaction_count;
        if ct.uint() + 1 == u32::MAX  {
            return errf!("transaction overflow")
        }
        *ct += 1;
        self.transactions.set_count(ct.uint() as u64);
        self.transactions.push(tx);
        Ok(())
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

