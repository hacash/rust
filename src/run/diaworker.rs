
use std::sync::atomic::{AtomicU32, Ordering::{self, Relaxed} };

use crate::mint::action::*;



// current mining diamond number
static MINING_DIAMOND_NUM: AtomicU32 = AtomicU32::new(0);
lazy_static!{
    static ref MINING_STUFF: Mutex<Hash> = Mutex::default();
    static ref MINING_SUCCESS: Mutex<Vec<DiamondMint>> = Mutex::default();
}



/*
* Diamond worker
*/
pub fn diaworker() {


    // config
    let cnfp = "./diaworker.config.ini".to_string();
    let inicnf = config::load_config(cnfp);
    let mut cnfobj = DiaWorkConf::new(&inicnf);

    // test start
    // cnfobj.supervene = 1;
    // test end

    // start
    start_diamond_worker(cnfobj);
}


macro_rules! loop_retry {
    ($sec: expr) => {
        sleep(Duration::from_secs($sec));
        continue // try to reconnect
    }
}

/*
* start
*/
fn start_diamond_worker(mut cnfobj: DiaWorkConf) {

    load_init(&mut cnfobj);

    start_all_miner_thread(&cnfobj);

    pull_and_push_loop(cnfobj)

}


fn start_all_miner_thread(cnf: &DiaWorkConf) {

    for i in 0 .. cnf.supervene {
        let thrid = i as usize;
        let addr = cnf.rewardaddr.clone();
        spawn(move || {
            start_one_worker_thread(thrid, addr)
        });
    }

}


// 
fn start_one_worker_thread(thrid: usize, res_addr: Address) {

    const target_mining_period: f64 = 5000.0; // 5 sec

    let mut nonce_space: u64 = 5000;
    let mut current_mining_number: u32 = 0;
    let mut current_mining_block_hash: Hash = Hash::default();

    loop {
        let cmdn = MINING_DIAMOND_NUM.load(Relaxed);
        if cmdn == 0 {
            loop_retry!(3); // not yet
        }
        if cmdn > current_mining_number {
            // next mining
            current_mining_number = cmdn;
            current_mining_block_hash = MINING_STUFF.lock().unwrap().clone();
        }
        // start mining
        let mut custom_nonce = Hash::default();
        getrandom::getrandom(custom_nonce.as_mut()).unwrap(); 
        let mut nonce_start = 0;

        loop {
            let ctn = Instant::now();
            let mres = do_diamond_group_mining(current_mining_number, &current_mining_block_hash,
                &res_addr, &custom_nonce,
                nonce_start, nonce_space, 
            );
            if let Some(success) = mres { // mining success
                MINING_SUCCESS.lock().unwrap().push(success); // send success
            }
            let use_times = Instant::now().duration_since(ctn);
            let ns = nonce_start.checked_add(nonce_space);
            if let None = ns {
                break // u64 nonce end
            }
            nonce_start = ns.unwrap();
            nonce_space = ( nonce_space as f64 / ( use_times.as_millis() as f64 / target_mining_period ) ) as u64;
            // next space
        }
        // next custom nonce
    }

}


fn do_diamond_group_mining(number: u32, prevblockhash: &Hash, 
    rwdaddr: &Address, custom_message: &Hash,
    nonce_start: u64, nonce_space: u64, 
) -> Option<DiamondMint> {
    let empthbytes = [0u8; 0];
    let prevhash: &[u8; x16rs::HASH_SIZE] = prevblockhash;
    let address: &[u8; 21] = rwdaddr;
    let custom_nonce: &[u8] = match number > DIAMOND_ABOVE_NUMBER_OF_CREATE_BY_CUSTOM_MESSAGE {
        true => custom_message.as_bytes(),
        false => &empthbytes,
    };
    // start mining
    for nonce in nonce_start .. nonce_start + nonce_space {
        let nonce_bytes = nonce.to_be_bytes();
        let (firhx, resxh, diastr) = x16rs::mine_diamond(number, prevhash, &nonce_bytes, address, custom_nonce);
        if let Some(dia_name) = x16rs::check_diamond_hash_result(&diastr) {
            // success find a diamond
            let name = DiamondName::cons(dia_name);
            let number = DiamondNumber::from(number);
            let mut diamint = DiamondMint::with(name, number);
            diamint.head.prev_hash = prevblockhash.clone();
            diamint.head.nonce = Fixed8::cons(nonce_bytes);
            diamint.head.address = rwdaddr.clone();
            diamint.custom_message = custom_message.clone();
            return Some(diamint)
        }
        // next
    }
    // finish with find nothing
    None
}




fn load_init(cnf: &mut DiaWorkConf) {

    let urlapi_pending = format!("http://{}/query/diamondminer/init", &cnf.rpcaddr);
    loop {
        let res = HttpClient::new().get(&urlapi_pending).send();
        let Ok(repv) = res else {
            println!("Error: cannot init diamond miner from {}", &urlapi_pending);
            loop_retry!(15);
        };
        let res: JV = serde_json::from_str(&repv.text().unwrap()).unwrap();
        let jstr = |k| { res[k].as_str().unwrap_or("") };
        let adr = jstr("reward_address");
        let Ok(rwd_addr) = Address::from_readable( &adr ) else {
            println!("Error: reward_address {} format error", &adr);
            loop_retry!(10);
        };
        // ok
        cnf.rewardaddr = rwd_addr;
        break
    }
    // ok

}



fn pull_and_push_loop(cnf: DiaWorkConf) {

    let urlapi_latest = format!("http://{}/query/latest", &cnf.rpcaddr);

    loop {
        // get next number
        let res = HttpClient::new().get(&urlapi_latest).send();
        let Ok(repv) = res else {
            println!("Error: cannot get latest from {}", &urlapi_latest);
            loop_retry!(15);
        };
        let res: JV = serde_json::from_str(&repv.text().unwrap()).unwrap();
        let jnum = |k| { res[k].as_u64().unwrap_or(0) };
        let next_num = jnum("number") as u32 + 1;
        let mining_num = MINING_DIAMOND_NUM.load(Relaxed);
        if next_num > mining_num {
            // turn to next!
            let urlapi_diamond = format!("http://{}/query/diamond?number={}", &cnf.rpcaddr, next_num - 1);
            let res = HttpClient::new().get(&urlapi_diamond).send();
            let Ok(repv) = res else {
                println!("Error: cannot get diamond from {}", &urlapi_diamond);
                loop_retry!(10);
            };
            let res: JV = serde_json::from_str(&repv.text().unwrap()).unwrap();
            let prev_hash = res["born"]["hash"].as_str().unwrap_or("");
            if let Ok(hx) = hex::decode(&prev_hash) {
                if hx.len() == x16rs::HASH_SIZE {
                    *MINING_STUFF.lock().unwrap() = Hash::cons(hx.try_into().unwrap());
                    MINING_DIAMOND_NUM.store(next_num, Relaxed);
                }
            }
        } 
        // check success
        if let Some(mint) = {
            let mut list = MINING_SUCCESS.lock().unwrap();
            let res = list.pop();
            list.clear();
            res
        } {
            // upload success
            let actionbody = mint.serialize();
            let urlapi_success = format!("http://{}/submit/diamondminer/success", &cnf.rpcaddr);
            let res = HttpClient::new().post(&urlapi_success).body(actionbody).send();
            if let Ok(repv) = res {
                println!("\n----------------\n- ✩✩✩ DIAMOND MINING SUCCESS: {}({}) \n----------------\n", 
                    mint.head.diamond.readable(), mint.head.number.uint(),
                );
            }
        }






        loop_retry!(3); // waiting
    }




}




