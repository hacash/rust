use  std::time::{ Duration, Instant };


impl Engine for BlockEngine {


    fn insert(&self, blkpkg: Box<dyn BlockPkg>) -> RetErr {    
        self.isrlck.lock();
        let mut times = vec![
            Duration::new(0, 0),
            Duration::new(0, 0),
            Duration::new(0, 0),
            Duration::new(0, 0),
            Duration::new(0, 0),
        ];
        self.insert_unsafe(blkpkg, &mut times)
    }


    fn insert_sync(&self, start_hei: u64, mut datas: Vec<u8>) -> RetErr {
        self.isrlck.lock();
        let mut times = vec![
            Duration::new(0, 0),
            Duration::new(0, 0),
            Duration::new(0, 0),
            Duration::new(0, 0),
            Duration::new(0, 0),
        ];
        let mut hei = start_hei;
        let mut blocks = datas.as_mut_slice();
        loop {
            if blocks.len() == 0 {
                break // end
            }
            let now0 = Instant::now();

            let blk = protocol::block::create(&blocks);
            if let Err(e) = blk {
                print_sync_warning(format!("blocks::create error: {}", e));
                break
            }
            let (blk, sk) = blk.unwrap();
            let blkhei = blk.height().uint();
            if hei != blkhei {
                print_sync_warning(format!("need block height {} but got {}", hei, blkhei));
                break
            }
            // 
            let (left, right) = blocks.split_at_mut(sk);
            blocks = right; // next chunk
            let pkg = BlockPackage::new_with_data(blk, left.into());
            
            let now1 = std::time::Instant::now();
            times[0] += now1.duration_since(now0);

            // do insert
            self.insert_unsafe(Box::new(pkg), &mut times);
            // next block
            hei += 1;
        } 

        // print time
        print!("< {:?}, {:?}, {:?}, {:?}, {:?} >", 
        times[0], times[1], times[2], times[3], times[4]);

        Ok(())
    }

}


impl BlockEngine {





    fn insert_unsafe(&self, blkpkg: Box<dyn BlockPkg>, times: &mut Vec<Duration>) -> RetErr {  

        let now0 = Instant::now();

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

        let now1 = Instant::now();
        times[1] += now1.duration_since(now0);
        // print!("{:?}, ", now1.duration_since(now0));

        // try insert
        let sub_state = do_check_insert(
            &self.cnf, 
            self.vmobj.as_ref(),
            self.mintk.as_ref(),
            base_chunk.state.clone(),
            base_chunk.block.objc().as_ref(),
            blkpkg.as_ref(),
        )?;
        let state_ptr = Arc::new(sub_state);


        let now2 = Instant::now();
        times[2] += now2.duration_since(now1);
        // print!("{:?}, ", now2.duration_since(now1));

        // append chunk
        let mut new_chunk = RollChunk::create(blkpkg, state_ptr);
        new_chunk.set_parent(base_chunk.clone());
        let chunk_ptr = Arc::new(new_chunk);
        base_chunk.push_child(chunk_ptr.clone());

        // if do roll and flush to disk
        let mut roll_root = self.klctx.lock().unwrap();
        let status = do_roll( &self.cnf, &mut roll_root, chunk_ptr.clone())?;
        // println!("{:?}", status);

        let now3 = Instant::now();
        times[3] += now3.duration_since(now2);
        // print!("{:?}, ", now3.duration_since(now2));

        // do store
        do_store(&self.cnf, self.store.as_ref(), &mut roll_root, chunk_ptr, status);

        let now4 = Instant::now();
        times[4] += now4.duration_since(now3);
        // println!("{:?}", now4.duration_since(now3));


        Ok(())

    }

}





fn print_sync_warning(e: String) {
    println!("\n\n[Block Sync Warning] {}\n\n", e);
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