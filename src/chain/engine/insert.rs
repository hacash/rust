

impl Engine for BlockEngine {

    fn insert(&self, blkpkg: Box<dyn BlockPkg>) -> RetErr {    
        self.isrlck.lock();

        // search base chunk
        let prev_hx = blkpkg.objc().prevhash();
        let base_chunk = {
            let roll_root = self.klctx.lock().unwrap();
            let bsck = locate_base_chunk(&roll_root, prev_hx);
            if let None = bsck {
                return errf!("not find prev hash {}", prev_hx)
            }
            bsck.unwrap()
        };

        // try insert
        let sub_state = do_check_insert(
            &self.cnf, 
            self.mintk.as_ref(),
            base_chunk.state.clone(),
            base_chunk.block.objc().as_ref(),
            blkpkg.as_ref(),
        ) ? ;
        let state_ptr = Arc::new(sub_state);

        // append chunk
        let new_chunk = RollChunk::create(blkpkg, state_ptr);
        new_chunk.set_parent(base_chunk.clone());
        let chunk_ptr = Arc::new(new_chunk);
        base_chunk.push_child(chunk_ptr.clone());

        // if do roll and flush to disk
        let mut roll_root = self.klctx.lock().unwrap();
        do_roll( &self.cnf, &mut roll_root, chunk_ptr)

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
    }

}
