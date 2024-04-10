

impl Engine for BlockEngine {

    fn insert(&self, blkpkg: Box<dyn BlockPkg>) -> RetErr {    
        self.isrlck.lock();
        // lock
        let rollres;
        {
            let ctx = self.klctx.lock().unwrap();
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
    }

}
