

const HACASH_NODE_VERSION: &str = "0.1.2";
const HACASH_NODE_BUILD_TIME: &str = "2024.9.15-1";
const HACASH_STATE_DB_UPDT: u32 = 1;


struct EmptyBlockScaner {}
impl BlockScaner for EmptyBlockScaner {}



pub fn fullnode() {
    fullnode_with(None)
}


pub fn fullnode_with(blkscaner: Option<Box<dyn BlockScaner>>) {

    // config
    let cnfp = "./hacash.config.ini".to_string();
    let inicnf = config::load_config(cnfp);

    // block scaner
    let scaner = init_block_scaner(&inicnf, blkscaner);

    // hacash node
    start_hacash_node(inicnf, scaner);
}

/*
* init block scaner
*/
fn init_block_scaner(inicnf: &IniObj, blkscaner: Option<Box<dyn BlockScaner>>) -> Arc<dyn BlockScaner> {

    // scaner
    let scaner: Arc<dyn BlockScaner> = match blkscaner {
        Some(mut scan) => {
            scan.init(inicnf).unwrap(); // init block scaner
            scan.into()
        },
        _ => Arc::new(EmptyBlockScaner{}),
    };

    // start block scaner
    let scanercp1 = scaner.clone();
    std::thread::spawn(move||{
        scanercp1.start().unwrap();
    });
    let scanercp2 = scaner.clone();
    std::thread::spawn(move||{
        scanercp2.serve().unwrap();
    });
    
    // ok
    scaner
}


/*
* create and start hash node
*/
fn start_hacash_node(iniobj: IniObj, blkscaner: Arc<dyn BlockScaner>) {

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
    let engine = BlockEngine::open(&iniobj, dbv, mint_checker, blkscaner.clone());
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
        blkscaner.exit();
        hn2.close(); // ctrl+c to quit
    }});

    // start
    HacashNode::start(hnode);

    // on closed
    println!("\nHacash node closed.");
}







