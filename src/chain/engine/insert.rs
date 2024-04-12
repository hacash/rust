


impl Engine for BlockEngine {


    fn insert(&self, blkpkg: Box<dyn BlockPkg>) -> RetErr {    
        self.isrlck.lock();
        self.insert_unsafe(blkpkg)
    }

}


impl BlockEngine {

    fn insert_unsafe(&self, blkpkg: Box<dyn BlockPkg>) -> RetErr {  
        // search base chunk
        let blk_hei = blkpkg.objc().height();
        let blk_hash = blkpkg.hash();
        let prev_hx = blkpkg.objc().prevhash();
        let base_chunk = {
            let roll_root = self.klctx.lock().unwrap();
            let bsck = locate_base_chunk(&roll_root, prev_hx);
            if let None = bsck {
                return errf!("not find prev hash {}", prev_hx)
            }
            bsck.unwrap()
        };
        // check repeat
        for sub in base_chunk.childs.borrow().iter() {
            if *blk_hash == sub.hash {
                return errf!("repetitive block height {} hash {}", blk_hei.to_u64(), blk_hash)
            }
        }
        // try insert
        let sub_state = do_check_insert(
            &self.cnf, 
            self.vmobj.as_ref(),
            self.mintk.as_ref(),
            base_chunk.state.clone(),
            base_chunk.block.objc().as_ref(),
            blkpkg.as_ref(),
        ) ? ;
        let state_ptr = Arc::new(sub_state);

        // append chunk
        let mut new_chunk = RollChunk::create(blkpkg, state_ptr);
        new_chunk.set_parent(base_chunk.clone());
        let chunk_ptr = Arc::new(new_chunk);
        base_chunk.push_child(chunk_ptr.clone());

        // if do roll and flush to disk
        let mut roll_root = self.klctx.lock().unwrap();
        let status = do_roll( &self.cnf, &mut roll_root, chunk_ptr.clone()) ? ;
        // println!("{:?}", status);

        // do store
        do_store(&self.cnf, self.store.as_ref(), &mut roll_root, chunk_ptr, status)

    }

}








/*
// lock
let rollres;
{
    // do insert
    let (bsck, state) = do_insert(self, &self.cnf, &ctx, self.mintk.as_ref(), blkpkg.as_ref()) ? ;
    // insert success try do roll
    rollres = do_roll(&self.cnf, &ctx, blkpkg, bsck, state) ? ;
}
if let Some((scusp, state, sroot)) = rollres {
    // change ptr
    let mut ctx = self.klctx.lock().unwrap();
    do_roll_chunk_state(&mut ctx, scusp, state, sroot) ? ;
}
// ok finish 
Ok(())
*/