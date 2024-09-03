

use crate::mint::difficulty::*;



const STPSEC: u64 = 3; // 3 secs

const API_NOTICE: &str = "http://{}/query/miner/notice?height={}&rpid={}";
const API_PENDING: &str = "http://{}/query/miner/pending?";
const API_SUCCESS: &str = "http://{}/submit/miner/success?";



#[derive(Clone, Default)]
struct BlockStuff {
    height: u64,
    // mrng_start: u32,
    // mrng_end: u32,
    coinbase_nonce: Vec<u8>,
    target_hash: Vec<u8>,
    block_intro: Vec<u8>,
}

#[derive(Clone, Default)]
struct MiningResult {
    stuff: Arc<BlockStuff>,
    space: u32,
    nonce: u32,
    mnper: u16,
    hash: Vec<u8>,
}

#[derive(Clone, Default)]
struct MiningSuccess {
    stuff: Arc<BlockStuff>,
    nonce: u32,
    hash: Vec<u8>,
}


#[derive(Clone, Default)]
struct MostPower {
    hash: Vec<u8>,
    secs: u64, // time secs
}
impl MostPower {
    fn clear(&mut self) {
        self.hash = vec![255].repeat(32);
        self.secs = 0;
    }
}



type MostPwr = Arc<Mutex<MostPower>>;

type StuffSender = Arc<Mutex<spmc::Sender<Arc<BlockStuff>>>>;


////////////////////////////////////////////////////




pub fn poworker() {


    // config
    let cnfp = "./poworker.config.ini".to_string();
    let inicnf = config::load_config(cnfp);
    let mut cnfobj = PoWorkConf::new(&inicnf);

    // test start
    // cnfobj.supervene = 1;
    // cnfobj.noncemax = u32::MAX / 200;
    // cnfobj.noticewait = 5;
    // test end


    // start
    start_pow_worker(cnfobj);
}


fn start_pow_worker(cnf: PoWorkConf) {

    // let mut current_height = 0;

    let mostpwr: MostPwr = Arc::default();
    {
        mostpwr.lock().unwrap().clear(); // default
    }

    let (print_tx, print_rx) = mpsc::channel();
    let (stuff_tx, stuff_rx) = spmc::channel::<Arc<BlockStuff>>();
    let stuff_tx = Arc::new(Mutex::new(stuff_tx));
    let (reslt_tx, reslt_rx) = mpsc::channel(); // mining result
    let (fnish_tx, fnish_rx) = mpsc::channel(); // cycle finish 
    

    // pull new block from full node rpc api
    {
        let cnf = cnf.clone();
        let prtx = print_tx.clone();
        let sftx = stuff_tx.clone();
        let mstp = mostpwr.clone();
        spawn(move || {
            pull_new_block(cnf, mstp, prtx, sftx);
        });
    }

    // start pow workers
    {
        let cnf = cnf.clone();
        let prtx = print_tx.clone();
        let sfrx = stuff_rx.clone();
        let rstx = reslt_tx.clone();
        let fntx = fnish_tx.clone();
        spawn(move || {
            start_pow_workers(cnf, prtx, sfrx, rstx, fntx);
        });
    }

    // get all result
    {
        let cnf = cnf.clone();
        let prtx = print_tx.clone();
        let mstp = mostpwr.clone();
        spawn(move || {
            get_all_results(cnf, mstp, prtx, reslt_rx);
        });

    }

    // listen finish
    {
        let cnf = cnf.clone();
        let prtx = print_tx.clone();
        let sftx = stuff_tx.clone();
        let mstp = mostpwr.clone();
        spawn(move || {
            listen_finish_event(cnf, mstp, prtx, fnish_rx, sftx)
        });
    }



    // print all notes
    drop(print_tx);
    let mut termwide = || termsize::get().unwrap().cols as usize - 1;
    let mut prev_line_is_floating: bool = false;
    for mut line in print_rx {
        let clisflt: bool = !line.ends_with("\n");
        print!("\r{}\r", " ".repeat( termwide() ));
        match clisflt {
            true => flush!("\r{}", line),
            false => match prev_line_is_floating {
                true => flush!("\r{}", line),
                false => flush!("{}", line),
            },
        }
        // next
        prev_line_is_floating = clisflt;
    }

}



/*


    ctrlc::set_handler(move||{
        // println!("ctrl+c to quit");
        std::process::exit(0);
    }); // ctrl+c to quit


*/


////////////////////////////////////////////////////



/*
* 
*/
fn get_all_results(cnf: PoWorkConf, 
    mostpwr: MostPwr,
    prtx: mpsc::Sender<String>,
    rsrx: mpsc::Receiver<MiningResult>,
) {
    let mut mnres = MiningResult::default();
    loop {
        let mut power_space = 0u32;
        let mut power_nonce = 0u32;
        let mut power_hash = vec![255u8].repeat(32);
        for i in 0..cnf.supervene {
            let res = rsrx.recv().unwrap();
            if power_big_than(&res.hash, &power_hash) {
                power_hash = res.hash.clone();
                power_nonce = res.nonce;
                power_space = res.space;
            }
            mnres = res;
        }
        // check success
        if power_big_than(&power_hash, &mnres.stuff.target_hash) {
            // TODO: mining success
            push_mining_success(&cnf, MiningSuccess{
                stuff: mnres.stuff.clone(),
                nonce: power_nonce,
                hash: power_hash.clone(),
            }, prtx.clone());
        }
        {
            let mut pwr = mostpwr.lock().unwrap();
            pwr.secs += STPSEC;
            if power_big_than(&power_hash, &pwr.hash) {
                pwr.hash = power_hash.clone();
            }
        }
        // print
        let mnper = mnres.mnper as f32 / 100.0;
        let hashrate = hash_to_rateshow(&power_hash[0..32].try_into().unwrap(), STPSEC); // 3s
        prtx.send(format!("do mining {} {}... {} ({}%) {}", mnres.stuff.height, 
            hex::encode(&power_hash[0..10]), power_space, mnper, hashrate,
        )).unwrap();
    }
}



/*
* 
*/
fn start_pow_workers(cnf: PoWorkConf, 
    prtx_p: mpsc::Sender<String>, 
    sfrx_p: spmc::Receiver<Arc<BlockStuff>>, 
    rstx_p: mpsc::Sender<MiningResult>,
    fntx_p: mpsc::Sender<u8>,
) {

    // start worker
    for i in 0..cnf.supervene {
        let thid = i;
        let prtx = prtx_p.clone();
        let sfrx = sfrx_p.clone();
        let rstx = rstx_p.clone();
        let fntx = fntx_p.clone();
        let vene = cnf.supervene;
        spawn(move || {
            let nonce_span = cnf.noncemax / vene; // u32::MAX / vene
            let nonce_start = nonce_span * thid;
            let nonce_end = nonce_start + nonce_span;
            let mut step_group = 50000u32;
            let mut step: u64 = 0;
            let mut stfinit = false;
            let mut stfobj = Arc::new(BlockStuff::default());
            let mut nonce_offset = nonce_start;
            loop {
                if let Ok(st) = sfrx.try_recv() {
                    stfobj = st;
                    stfinit = true;
                    nonce_offset = nonce_start;
                }
                if !stfinit {
                    sleep(Duration::from_millis(333));
                    continue
                }
                // do mining
                let (tms, nn, nhx) = do_group_mining(stfobj.height, nonce_offset, step_group, stfobj.block_intro.clone(), stfobj.clone());
                // println!("#{} mining height {} step_group {} result {} {} in {}ms", 
                //     thid, stfobj.height, step_group, nn, nhx.hex(), tms);
                let mnper = ((nonce_offset - nonce_start) as f32 / nonce_span as f32 * 10000.0) as u16;
                rstx.send(MiningResult {
                    stuff: stfobj.clone(),
                    space: step_group,
                    nonce: nn,
                    mnper: mnper,
                    hash: nhx,
                }).unwrap();
                // sleep(Duration::from_secs(1));
                // next
                let mss = STPSEC as f64 * 1000.0;
                step_group = (step_group as f64 * mss / tms) as u32; // target time = 3s
                let next_offset = nonce_offset + step_group;
                if next_offset >= nonce_end - 1{
                    fntx.send(1).unwrap();
                    stfinit = false; // finish and reinit
                }
                nonce_offset = next_offset;
            }
        });

    }



}



/*
* return: time ms, nonce & most power hash
*/
fn do_group_mining(hei: u64, start: u32, cnum: u32, 
    mut intro: Vec<u8>, stuff: Arc<BlockStuff>,
) -> (f64, u32, Vec<u8>) {

    let mut power_nonce = 0u32;
    let mut power_hash = vec![255u8].repeat(32);
    let ctn = Instant::now();
    for nonce in start..start+cnum {
        intro[79..83].copy_from_slice(&nonce.to_be_bytes());
        let reshx = x16rs::block_hash(hei, &intro).to_vec();
        if power_big_than(&reshx, &power_hash) {
            power_hash = reshx;
            power_nonce = nonce;
        }
    }
    let useti = Instant::now().duration_since(ctn).as_millis();
    // ok
    (useti as f64, power_nonce, power_hash)
}




/*
*
*/
fn broadcast_new_block(cnf: &PoWorkConf, res: &JV, 
    mostpwr: MostPwr,
    prtx: mpsc::Sender<String>,
    stuff_tx: StuffSender,
) {
    {
        let mut pwr = mostpwr.lock().unwrap();
        if pwr.secs > 0 {
            let hashrate = hash_to_rateshow(&pwr.hash[0..32].try_into().unwrap(), pwr.secs);
            prtx.send(format!("hashrates {}/{}s = {}\n", 
                pwr.hash[..13].to_vec().hex(), pwr.secs, hashrate,
            )).unwrap();
            pwr.clear(); // clear
        }
    }
    let jstr = |k| { res[k].as_str().unwrap_or("") };
    let jnum = |k| { res[k].as_u64().unwrap_or(0) };
    let curhei = jnum("height");
    let cbnc = hex::decode(jstr("coinbase_nonce")).unwrap();
    let tarhxstr = jstr("target_hash");
    let tarhx = hex::decode(tarhxstr).unwrap();
    let tarhxshort = tarhxstr.trim_end_matches('f').trim_end_matches('0');
    prtx.send(format!("req block {} cbn ...{} tarhx {}...\n", curhei, 
        cbnc[28..].to_vec().hex(), tarhxshort,
    )).unwrap();
    // notice all workers
    let stuff = BlockStuff {
        height: curhei,
        coinbase_nonce: cbnc,
        target_hash: tarhx,
        block_intro: hex::decode(jstr("block_intro")).unwrap(),
    };
    let stfptr = Arc::new(stuff);
    let mut stx = stuff_tx.lock().unwrap();
    for _ in 0..cnf.supervene {
        stx.send(stfptr.clone()).unwrap(); // broadcast to all workers
    }
}




/*
* 
*/
fn listen_finish_event(cnf: PoWorkConf, 
    mostpwr: MostPwr,
    prtx: mpsc::Sender<String>,
    fnrx: mpsc::Receiver<u8>,
    stuff_tx: StuffSender,
) {
    loop {
        fnrx.recv().unwrap(); // wait
        // do req
        let urlapi_pending = format!("http://{}/query/miner/pending", &cnf.rpcaddr);
        let res = HttpClient::new().get(&urlapi_pending).send();
        if let Ok(repv) = res {
            if let Ok(res) = serde_json::from_str(&repv.text().unwrap()) {
                // println!("listen_finish_event - broadcast_new_block - {}", &urlapi_pending);
                broadcast_new_block(&cnf, &res, mostpwr.clone(), prtx.clone(), stuff_tx.clone());
            }
        };
        // ignore other
        for _ in 0..cnf.supervene-1 {
            fnrx.recv().unwrap();
        }
    }
}




/*
*
*/
fn pull_new_block(cnf: PoWorkConf, 
    mostpwr: MostPwr,
    prtx: mpsc::Sender<String>,
    stuff_tx: StuffSender,
) {
    let urlapi_pending = format!("http://{}/query/miner/pending", &cnf.rpcaddr);
    let mut current_height = 0;
    loop {
        macro_rules! retry {
            () => {
                sleep(Duration::from_secs(15));
                continue // try to reconnect
            }
        }
        // read cur blk stuff
        let res = HttpClient::new().get(&urlapi_pending).send();
        let Ok(repv) = res else {
            println!("Error: cannot get block data at {}", &urlapi_pending);
            retry!();
        };
        let res: JV = serde_json::from_str(&repv.text().unwrap()).unwrap();
        let jstr = |k| { res[k].as_str().unwrap_or("") };
        let jnum = |k| { res[k].as_u64().unwrap_or(0) };
        let JV::String(ref blkhd) = res["block_intro"] else {
            println!("Error: get block stuff error: {}", jstr("err") );
            retry!();
        };
        current_height = jnum("height");
        // new block
        // println!("pull_new_block - broadcast_new_block - {}", &urlapi_pending);
        broadcast_new_block(&cnf, &res, mostpwr.clone(), prtx.clone(), stuff_tx.clone());
        // wait notice
        let mut rpid = vec![0].repeat(16);
        loop {
            getrandom::getrandom(&mut rpid).unwrap();
            let urlapi_notice = format!("http://{}/query/miner/notice?wait={}&height={}&rqid={}", 
            &cnf.rpcaddr, &cnf.noticewait, current_height, &hex::encode(&rpid));
            // println!("\n-------- {} -------- {}\n", &ctshow(), &urlapi_notice);
            let res = HttpClient::new().get(&urlapi_notice).timeout(Duration::from_secs(99999)).send();
            let Ok(repv) = res else {
                println!("Error: cannot get miner notice at {}", &urlapi_notice);
                retry!();
            };
            let  Ok(jsdata) = repv.text() else {
                println!("Error: cannot read miner notice at {}", &urlapi_notice);
                retry!();
            };
            let Ok(res2) = serde_json::from_str::<JV>(&jsdata) else {
                // println!("{}", &jsdata);
                panic!("miner notice error: {}", &jsdata);
            };
            let jnum = |k| { res2[k].as_u64().unwrap_or(0) };
            let res_hei = jnum("height");
            // println!("\n++++++++ {} {} {}\n", &jsdata, res_hei, current_height);
            if res_hei >= current_height {
                // next block discover
                break 
            }
            // continue to wait
        }
    }





}




/*
*
*/
fn push_mining_success(cnf: &PoWorkConf, 
    success: MiningSuccess,
    prtx: mpsc::Sender<String>,
) {
    let urlapi_success = format!(
        "http://{}/submit/miner/success?height={}&block_nonce={}&coinbase_nonce={}", 
        &cnf.rpcaddr, success.stuff.height, success.nonce, success.stuff.coinbase_nonce.hex()
    );
    HttpClient::new().get(&urlapi_success).send();
    // println!("{} {}", &urlapi_success, HttpClient::new().get(&urlapi_success).send().unwrap().text().unwrap());
    // print
    flush!("\n████████ [MINING SUCCESS] Find a block height {} hash {} to submit.\n",
        success.stuff.height, success.hash.hex()
    );
}


////////////////////////////////////////////////////


// hash_less_than
fn power_big_than(dst: &[u8], src: &[u8]) -> bool {
    let mut ln = dst.len();
    let l2 = src.len();
    if l2 < ln {
        ln = l2;
    }
    for i in 0..ln {
        let (l, r) = (dst[i], src[i]);
        if l < r {
            return true
        }else if l > r {
            return false
        }
    }
    return false
}