

const HACASH_NODE_VERSION: &str = "0.1.0";
const HACASH_NODE_BUILD_TIME: &str = "2024.8.1-1";
const HACASH_STATE_DB_UPDT: u32 = 1;


pub fn fullnode() {

    // config
    let cnfp = "./hacash.config.ini".to_string();
    let inicnf = config::load_config(cnfp);

    // start
    start_hacash_node(inicnf);
    
}


/*
* create and start hash node
*/
fn start_hacash_node(iniobj: sys::IniObj) {

    println!("[Version] full node v{}, build time: {}, database type: {}.", 
        HACASH_NODE_VERSION, HACASH_NODE_BUILD_TIME, HACASH_STATE_DB_UPDT
    );

    use std::sync::mpsc::channel;
    let (cltx, clrx) = channel();
    ctrlc::set_handler(move || cltx.send(()).unwrap()); // ctrl+c to quit

    // println!("startHacashNode ini={:?}", iniobj);
    // mint
    crate::mint::action::init_reg();

    let mint_checker = Box::new(BlockMintChecker::new(&iniobj));

    // engine
    let dbv = HACASH_STATE_DB_UPDT;
    let engine = BlockEngine::open(&iniobj, dbv, mint_checker);
    let engptr: Arc<BlockEngine> = Arc::new(engine);

    // node
    let mut hnode = Arc::new(HacashNode::open(&iniobj, engptr.clone()));

    // server
    let server = DataServer::open(&iniobj, engptr.clone(), hnode.clone());
    std::thread::spawn(move||{
        server.start(); // http rpc 
    });

    // handle ctr+c to close
    let hn2 = hnode.clone();
    std::thread::spawn(move||{ loop{
        clrx.recv();
        hn2.close(); // ctrl+c to quit
    }});

    // start
    HacashNode::start(hnode);

    // on closed
    println!("\nHacash node closed.");
}







