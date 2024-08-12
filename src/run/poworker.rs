

use reqwest::blocking::Client as HttpClient;
use serde_json::{ Value as JV };


use crate::mint::difficulty::*;




const API_NOTICE: &str = "http://{}/miner/notice?height={}&rpid={}";
const API_PENDING: &str = "http://{}/miner/pending?";
const API_SUCCESS: &str = "http://{}/miner/success?";



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
    hash: Vec<u8>,
}


type StuffSender = Arc<Mutex<spmc::Sender<Arc<BlockStuff>>>>;


////////////////////////////////////////////////////




pub fn poworker() {


    // config
    let cnfp = "./poworker.config.ini".to_string();
    let inicnf = config::load_config(cnfp);
    let mut cnfobj = PoWorkConf::new(&inicnf);

    // test start
    cnfobj.supervene = 2;
    cnfobj.noncemax = u32::MAX / 10;
    // test end


    // start
    start_pow_worker(cnfobj);
}


fn start_pow_worker(cnf: PoWorkConf) {

    // let mut current_height = 0;

    let (print_tx, print_rx) = mpsc::channel();
    let (stuff_tx, stuff_rx) = spmc::channel::<Arc<BlockStuff>>();
    let stuff_tx = Arc::new(Mutex::new(stuff_tx));
    let (reslt_tx, reslt_rx) = mpsc::channel(); // mining result
    let (fnish_tx, fnish_rx) = mpsc::channel(); // cycle finish 
    

    // pull new block from full node rpc api
    {
        let cnf = cnf.clone();
        let sftx = stuff_tx.clone();
        spawn(move || {
            pull_new_block(cnf, sftx);
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
        spawn(move || {
            get_all_results(cnf, prtx, reslt_rx);
        });

    }

    // listen finish
    {
        let cnf = cnf.clone();
        let sftx = stuff_tx.clone();
        spawn(move || {
            listen_finish_event(cnf, fnish_rx, sftx)
        });
    }



    // print all notes
    drop(print_tx);
    let mut prev_line_is_floating: bool = false;
    for line in print_rx {
        let clisflt: bool = !line.ends_with("\n");
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
        }

        // print
        let hashrate = hash_to_rateshow(&power_hash[0..32].try_into().unwrap(), 3); // 3s
        prtx.send(format!("mining {} - {}... ({}) {}", mnres.stuff.height, 
            hex::encode(&power_hash[0..16]), power_space, hashrate,
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
                rstx.send(MiningResult {
                    stuff: stfobj.clone(),
                    space: step_group,
                    nonce: nn,
                    hash: nhx,
                }).unwrap();
                // sleep(Duration::from_secs(1));
                // next
                step_group = (step_group as f64 * 3000.0 / tms) as u32; // target time = 3s
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
fn broadcast_new_block(cnf: &PoWorkConf, res: &JV, stuff_tx: StuffSender) {

    let jstr = |k| { res[k].as_str().unwrap_or("") };
    let jnum = |k| { res[k].as_u64().unwrap_or(0) };
    let stuff = BlockStuff {
        height: jnum("height"),
        coinbase_nonce: hex::decode(jstr("coinbase_nonce")).unwrap(),
        target_hash: hex::decode(jstr("target_hash")).unwrap(),
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
                println!("listen_finish_event - broadcast_new_block - {}", &urlapi_pending);
                broadcast_new_block(&cnf, &res, stuff_tx.clone());
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
    stuff_tx: StuffSender) {
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
        println!("pull_new_block - broadcast_new_block - {}", &urlapi_pending);
        broadcast_new_block(&cnf, &res, stuff_tx.clone());
        // wait notice
        let next_hei = current_height + 1;
        let mut rpid = vec![0].repeat(16);
        loop {
            getrandom::getrandom(&mut rpid).unwrap();
            let urlapi_notice = format!("http://{}/query/miner/notice?height={}&rpid={}", &cnf.rpcaddr, next_hei, &hex::encode(&rpid));
            let res = HttpClient::new().get(&urlapi_notice).send();
            let Ok(repv) = res else {
                println!("Error: cannot get miner notice at {}", &urlapi_notice);
                retry!();
            };
            let res: JV = serde_json::from_str(&repv.text().unwrap()).unwrap();
            let jnum = |k| { res[k].as_u64().unwrap_or(0) };
            let res_hei = jnum("height");
            if res_hei < next_hei {
                continue // continue to wait
            }
            // next block discover
            break
        }
    }





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