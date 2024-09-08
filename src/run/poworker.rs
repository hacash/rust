

const TARGET_BLOCK_TIME: f64 = 300.0; // 5 mins
const ONEDAY_BLOCK_NUM: f64 = 288.0; // one day block

// current mining diamond number
static MINING_BLOCK_HEIGHT: AtomicU64 = AtomicU64::new(0);
lazy_static!{
    static ref MINING_BLOCK_STUFF:  Mutex<Arc<BlockMiningStuff>> = Mutex::default();
    static ref MINING_BLOCK_RESULT: Mutex<Vec<BlockMiningResult>> = Mutex::default();
}



#[derive(Clone, Default)]
struct BlockMiningStuff {
    height: u64,
    target_hash: Hash,
    block_intro: BlockIntro,
    coinbase_tx: TransactionCoinbase,
    mkrl_list: Vec<Hash>,
}


#[derive(Clone, Default)]
struct BlockMiningResult {
    height: u64,
    nonce_space: u32,
    head_nonce: u32,
    coinbase_nonce: Vec<u8>,
    result_hash: Vec<u8>,
    target_hash: Vec<u8>,
}



pub fn poworker() {

    // config
    let cnfp = "./poworker.config.ini".to_string();
    let inicnf = config::load_config(cnfp);
    let mut cnf = PoWorkConf::new(&inicnf);

    // test start
    // cnfobj.supervene = 1;
    // cnfobj.noncemax = u32::MAX / 200;
    // cnfobj.noticewait = 5;
    // test end

    // deal results
    let cnf1 = cnf.clone();
    spawn(move || {
        let mut most_hash = vec![255u8; 32];
        loop {
            deal_block_mining_results(&cnf1, &mut most_hash);
            delay_continue_ms!(123);
        }
    });

    // start worker thread
    let thrnum = cnf.supervene;
    println!("\n[Start] Create #{} miner worker thread.", thrnum);
    for thrid in 0 .. cnf.supervene {
        let cnf2 = cnf.clone();
        spawn(move || {
            loop {
                run_block_mining_item(&cnf2, thrid);
                delay_continue_ms!(9);
            }
        });
    }

    // loop
    loop {
        pull_pending_block_stuff(&cnf);
        delay_continue_ms!(25);
    }
}



fn run_block_mining_item(cnf: &PoWorkConf, thrid: u32) {

    let mining_hei = MINING_BLOCK_HEIGHT.load(Relaxed);
    if mining_hei == 0 {
        delay_return_ms!(111); // not yet
    }

    let mut coinbase_nonce = Hash::default();
    getrandom::getrandom(coinbase_nonce.as_mut()).unwrap();
    let mut nonce_start: u32 = 0;
    let mut nonce_space: u32 = 100000;
    // stuff data
    let stuff = { MINING_BLOCK_STUFF.lock().unwrap().clone() };
    let height = stuff.height;
    let mut cbtx = stuff.coinbase_tx.clone();
    cbtx.set_nonce(coinbase_nonce);
    let mut block_intro = stuff.block_intro.clone();
    block_intro.set_mrklroot( calculate_mrkl_coinbase_update(cbtx.hash(), &stuff.mkrl_list) );
    // nonce total space = u32
    let mut nonce_finish = false;
    loop {
        let ctn = Instant::now();
        let (head_nonce, result_hash) = do_group_block_mining(height, block_intro.serialize(), nonce_start, nonce_space);
        let use_secs = Instant::now().duration_since(ctn).as_millis() as f64 / 1000.0;
        // record result
        let mlres = BlockMiningResult {
            height,
            nonce_space,
            head_nonce,
            coinbase_nonce: coinbase_nonce.to_vec(),
            result_hash: result_hash.to_vec(),
            target_hash: stuff.target_hash.to_vec(),
        };
        {
            let mut results = MINING_BLOCK_RESULT.lock().unwrap();
            results.push(mlres);
        }
        if nonce_finish {
            return // end u32 nonce
        }
        // update space
        nonce_space = (nonce_space as f64 * MINING_INTERVAL / use_secs) as u32;
        let Some(nst) = nonce_start.checked_add(nonce_space) else {
            nonce_finish = true;
            nonce_space = u32::MAX - nonce_start - 1;
            continue // // u32 nonce space finish
        };
        nonce_start = nst;
        // check next height
        let check_hei = MINING_BLOCK_HEIGHT.load(Relaxed);
        if check_hei > mining_hei {
            return // turn to next height
        }
        // continue nonce space
    }

}

// return: nonce, hash
fn do_group_block_mining(height: u64, mut block_intro: Vec<u8>, 
    nonce_start: u32, nonce_space: u32,
) -> (u32, [u8; 32]) {
    let mut most_nonce = 0u32;
    let mut most_hash = [255u8; 32];
    for nonce in nonce_start .. nonce_start + nonce_space {
        block_intro[79..83].copy_from_slice(&nonce.to_be_bytes());
        let reshx = x16rs::block_hash(height, &block_intro);
        if hash_more_power(&reshx, &most_hash) {
            most_hash = reshx;
            most_nonce = nonce;
        }
    }
    // end
    (most_nonce, most_hash)
}


fn deal_block_mining_results(cnf: &PoWorkConf, most_hash: &mut Vec<u8>) {
    let vene = cnf.supervene;
    let mut results = MINING_BLOCK_RESULT.lock().unwrap();
    let resnum = results.len() as u32;
    if resnum != vene {
        return // not yet
    }
    // deal
    let mut deal_hei = 0u64;
    let mut most = BlockMiningResult::default();
    most.result_hash = vec![255u8].repeat(32);
    let mut total_nonce_space = 0u64;
    for i in 0 .. resnum as usize {
        let res = &results[i];
        deal_hei = res.height;
        total_nonce_space += res.nonce_space as u64;
        if hash_more_power(&res.result_hash, &most.result_hash) {
            most = res.clone();
        }
    }
    results.clear();
    drop(results);
    // total most
    if hash_more_power(&most.result_hash, most_hash) {
        *most_hash = most.result_hash.clone();
    }
    // print hashrates
    let tarhx: [u8; HASH_WIDTH] = most.target_hash.clone().try_into().unwrap();
    let target_rates = hash_to_rates(&tarhx, TARGET_BLOCK_TIME);
    let nonce_rates = total_nonce_space as f64 / MINING_INTERVAL;
    let mut mnper = nonce_rates / target_rates;
    if mnper > 1.0 {
        mnper = 1.0;
    }
    let hac1day = mnper * ONEDAY_BLOCK_NUM * block_reward_number(deal_hei) as f64;
    flush!("{}-{:.6}%, {} {}, ≈{:.4}HAC/day, hashrates: {}.        \r", 
        total_nonce_space, mnper * 100.0,
        hex::encode(hash_left_zero_pad(&most.result_hash, 2)), 
        hex::encode(hash_left_zero_pad3(&most_hash)), 
        hac1day, rates_to_show(nonce_rates)
    );
    // check success
    if hash_more_power(&most.result_hash, &most.target_hash) {
        push_block_mining_success(cnf, &most);
    }
    // print next height
    may_print_turn_to_nex_block_mining(deal_hei, Some(most_hash));
}


fn may_print_turn_to_nex_block_mining(curr_hei: u64, most_hash: Option<&mut Vec<u8>>) {
    let mining_hei = MINING_BLOCK_HEIGHT.load(Relaxed);
    if curr_hei >= mining_hei {
        return // not turn
    }
    if let Some(most_hash) = most_hash {
        *most_hash = vec![255u8; 32]; // reset 
    }
    let stuff = MINING_BLOCK_STUFF.lock().unwrap();
    let tarhx = hash_left_zero_pad3(&stuff.target_hash.as_bytes()).hex();

    println!("\n[{}] req height {} target {} to mining ... ", 
        &ctshow()[5..], mining_hei, tarhx
    );
}


fn set_pending_block_stuff(height: u64, res: serde_json::Value) {
    let jstr = |k: &str| { res[k].as_str().unwrap_or("") };
    let jnum = |k: &str| { res[k].as_u64().unwrap_or(0) };
    // data
    // println!("{:?}", &res);
    let target_hash = Hash::cons(hex::decode(jstr("target_hash")).unwrap().try_into().unwrap());
    let block_intro = BlockIntro::must(&hex::decode(jstr("block_intro")).unwrap());
    let coinbase_tx = TransactionCoinbase::must(&hex::decode(jstr("coinbase_body")).unwrap());
    let mut mkrl_list = Vec::new();
    if let JV::Array(ref lists) = res["mkrl_modify_list"] {
        for li in lists {
            mkrl_list.push(Hash::cons(hex::decode(li.as_str().unwrap_or("")).unwrap().try_into().unwrap()));
        }
    }
    // set pending stuff
    let new_stuff = BlockMiningStuff{
        height,
        target_hash,
        block_intro,
        coinbase_tx,
        mkrl_list,
    };
    {
        let mut stuff = MINING_BLOCK_STUFF.lock().unwrap();
        *stuff = new_stuff.into();
    }
    MINING_BLOCK_HEIGHT.store(height, Relaxed);
}



///////////////////////////////




fn pull_pending_block_stuff(cnf: &PoWorkConf) {

    let curr_hei = MINING_BLOCK_HEIGHT.load(Relaxed);

    // query pending
    let urlapi_pending = format!("http://{}/query/miner/pending?stuff=true", &cnf.rpcaddr);
    let res = HttpClient::new().get(&urlapi_pending).send();
    let Ok(repv) = res else {
        println!("Error: cannot get block data at {}\n", &urlapi_pending);
        delay_return!(30);
    };
    let res: JV = serde_json::from_str(&repv.text().unwrap()).unwrap();
    let jstr = |k| { res[k].as_str().unwrap_or("") };
    let jnum = |k| { res[k].as_u64().unwrap_or(0) };
    let JV::String(ref blkhd) = res["block_intro"] else {
        println!("Error: get block stuff error: {}", jstr("err") );
        delay_return!(15);
    };
    let pending_height = jnum("height");

    // set pending block stuff
    if pending_height > curr_hei {
        set_pending_block_stuff(pending_height, res);
        if curr_hei == 0 {
            may_print_turn_to_nex_block_mining(curr_hei, None); // print first
        }
    }

    // with notice
    let mut rpid = vec![0].repeat(16);
    loop {

        getrandom::getrandom(&mut rpid).unwrap();
        let urlapi_notice = format!("http://{}/query/miner/notice?wait={}&height={}&rqid={}", 
            &cnf.rpcaddr, &cnf.noticewait, pending_height, &hex::encode(&rpid)
        );
        // println!("\n-------- {} -------- {}\n", &ctshow(), &urlapi_notice);
        let res = HttpClient::new().get(&urlapi_notice).timeout(Duration::from_secs(300)).send();
        let Ok(repv) = res else {
            println!("Error: cannot get miner notice at {}\n", &urlapi_notice);
            delay_return!(10);
        };
        let Ok(jsdata) = repv.text() else {
            println!("Error: cannot read miner notice at {}", &urlapi_notice);
            delay_return!(1);
        };
        let Ok(res2) = serde_json::from_str::<JV>(&jsdata) else {
            // println!("{}", &jsdata);
            panic!("miner notice error: {}", &jsdata);
        };
        let jnum = |k| { res2[k].as_u64().unwrap_or(0) };
        let res_hei = jnum("height");
        // println!("\n++++++++ {} {} {}\n", &jsdata, res_hei, current_height);
        if res_hei >= pending_height {
            // next block discover
            break 
        }
        // continue to wait

    }

}


fn push_block_mining_success(cnf: &PoWorkConf, success: &BlockMiningResult) {
    let urlapi_success = format!(
        "http://{}/submit/miner/success?height={}&block_nonce={}&coinbase_nonce={}", 
        &cnf.rpcaddr, success.height, success.head_nonce, success.coinbase_nonce.hex()
    );
    HttpClient::new().get(&urlapi_success).send();
    // println!("{} {}", &urlapi_success, HttpClient::new().get(&urlapi_success).send().unwrap().text().unwrap());
    // print
    println!("\n\n████████████████ [MINING SUCCESS] Find a block height {},\n██ hash {} to submit.",
        success.height, success.result_hash.hex()
    );
    println!("▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔")
}

