use  std::time::{ Duration, Instant };


impl Engine for BlockEngine {


    fn insert(&self, blkpkg: Box<dyn BlockPkg>) -> RetErr {    
        self.isrlck.lock();
        self.insert_unsafe(blkpkg)
    }


    fn insert_sync(&self, start_hei: u64, mut datas: Vec<u8>) -> RetErr {
        self.isrlck.lock();
        self.insert_sync_unsafe(start_hei, datas)
    }

}




impl BlockEngine {


    fn insert_sync_unsafe(&self, start_hei: u64, mut datas: Vec<u8>) -> RetErr {
        let (crtchin, crtchout) = std::sync::mpsc::sync_channel(10);
        let (istchin, istchout) = std::sync::mpsc::sync_channel(1);
        let (errchin, errchout) = std::sync::mpsc::sync_channel(5);
        let this = self;
        let errchin1 = errchin.clone();
        let errchin2 = errchin.clone();
        std::thread::scope(|s| {
            // parse block
            s.spawn(move || {
                let mut hei = start_hei;
                let mut blocks = datas.as_mut_slice();
                // let mut benchmark = Duration::new(0, 0);
                loop {
                    // let now = Instant::now();
                    if blocks.len() == 0 {
                        break
                    }
                    // let now0 = Instant::now();
                    let blk = block::create(&blocks);
                    if let Err(e) = blk {
                        let err = sync_warning(format!("blocks::create error: {}", e));
                        errchin1.send(err);
                        break
                    }
                    let (blk, sk) = blk.unwrap();
                    let blkhei = blk.height().uint();
                    if hei != blkhei {
                        let err = sync_warning(format!("need block height {} but got {}", hei, blkhei));
                        errchin1.send(err);
                        break
                    }
                    let (left, right) = blocks.split_at_mut(sk);
                    blocks = right; // next chunk
                    let mut pkg = Box::new(BlockPackage::new_with_data(blk, left.into()));
                    pkg.set_origin( block::BLOCK_ORIGIN::SYNC ); // mark block is sync
                    // let now1 = Instant::now();
                    // benchmark += now1.duration_since(now);
                    if let Err(_) = crtchin.send(pkg) {
                        break // end
                    }
                    // next block
                    hei += 1;
                }
                // print!(" {:?}", benchmark);
            });
            // create sub state
            s.spawn(move || {
                // let mut benchmark = Duration::new(0, 0);
                loop {
                    let blk = crtchout.recv();
                    if blk.is_err() {
                        break // end
                    }
                    // let now = Instant::now();
                    let blk = blk.unwrap();
                    let hei = blk.objc().height().uint();
                    let resck = this.exec_state(blk);
                    if resck.is_err() {
                        let err = sync_warning(format!("height {} exec state error: {}", 
                            hei, resck.err().unwrap()));
                        errchin2.send(err);
                        break // end
                    }
                    let chunk_ptr = resck.unwrap();
                    // let now1 = Instant::now();
                    // benchmark += now1.duration_since(now);
                    // next
                    if let Err(_) = istchin.send(chunk_ptr) {
                        break // end
                    }
                }
                // print!(" {:?}", benchmark);
            });
            // roll store
            // let mut benchmark = Duration::new(0, 0);
            // let mut lpnum = 0;
            loop {
                // lpnum += 1;
                let chk = istchout.recv();
                if chk.is_err() {
                    break // end
                }
                // let now = Instant::now();
                let chunk_ptr = chk.unwrap();
                let res = self.roll_store(chunk_ptr);
                if res.is_err() {
                    let err = sync_warning(format!("roll store error: {}", res.err().unwrap()));
                    errchin.send(err);
                    break // end
                }
                // next
                // let now1 = Instant::now();
                // benchmark += now1.duration_since(now);
            }
            // print!(" {:?} loop{} ", benchmark, lpnum);
            // ok not err
            errchin.send("".to_string());
        });
        let err = errchout.recv().unwrap();
        if err.len() > 0 {
            return Err(err)
        }
        // finish
        Ok(())
    }


    fn insert_unsafe(&self, blkpkg: Box<dyn BlockPkg>) -> RetErr {  
        let chunk_ptr = self.exec_state(blkpkg)?;
        self.roll_store(chunk_ptr)
    }


    fn exec_state(&self, blkpkg: Box<dyn BlockPkg>) -> Ret<Arc<RollChunk>> {
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
        for sub in base_chunk.childs.lock().unwrap().iter() {
            if *blk_hash == sub.hash {
                return errf!("repetitive block height {} hash {}", blk_hei.to_u64(), blk_hash)
            }
        }
        // try insert
        let sub_state = do_check_insert(
            &self.cnf, 
            self.mintk.as_ref(),
            self.store.as_ref(),
            base_chunk.state.clone(),
            base_chunk.block.objc().as_ref(),
            blkpkg.as_ref(),
        )?;
        let state_ptr = Arc::new(sub_state);
        // append chunk
        let mut new_chunk = RollChunk::create(blkpkg, state_ptr);
        new_chunk.set_parent(base_chunk.clone());
        let chunk_ptr = Arc::new(new_chunk);
        base_chunk.push_child(chunk_ptr.clone());
        // ok
        Ok(chunk_ptr)
    }

    fn roll_store(&self, chunk_ptr: Arc<RollChunk> ) -> RetErr {
        // if do roll and flush state to disk
        let mut roll_root = self.klctx.lock().unwrap();
        let status = do_roll( &self.cnf, &mut roll_root, chunk_ptr.clone())?;
        // println!("{:?}", status);
        // std::thread::sleep(std::time::Duration::from_millis(2)); // test
        do_store(&self.cnf, self.store.as_ref(), &mut roll_root, chunk_ptr, status)?;
        // do store
        Ok(())
    }

}



fn sync_warning(e: String) -> String {
    format!("\n\n[Block Sync Warning] {}\n\n", e)
}



/*
// lock
let rollres;
{
    // do insert
    let (bsck, state) = do_insert(self, &self.cnf, &ctx, self.mintk.as_ref(), blkpkg.as_ref())?;
    // insert success try do roll
    rollres = do_roll(&self.cnf, &ctx, blkpkg, bsck, state)?;
}
if let Some((scusp, state, sroot)) = rollres {
    // change ptr
    let mut ctx = self.klctx.lock().unwrap();
    do_roll_chunk_state(&mut ctx, scusp, state, sroot)?;
}
// ok finish 
Ok(())
*/