

pub struct NodeConf {
    pub node_id: [u8; 16],
    pub node_name: String,
    pub boot_nodes: Vec<SocketAddr>,
}


impl NodeConf {
    pub fn new(ini: &IniObj) -> NodeConf {
        // datadir
        let data_dir = get_datadir(ini);
        let nidfp = data_dir + "/node.id";
        
        // node id
        let mut node_id = [0u8; 16];
        let mut nidfile = OpenOptions::new().read(true).write(true).create(true).open(nidfp).expect("cannot open node info file.");
        // read
        let mut snid = String::new();
        nidfile.read_to_string(&mut snid);
        if let Ok(nid) = hex::decode(&snid) {
            if nid.len() == 16 {
                node_id = nid.try_into().unwrap();
            }
        }
        if node_id[0] == 0 {
            // save
            getrandom::getrandom(&mut node_id);
            nidfile.write_all(hex::encode(&node_id).as_bytes());
        }
        let nidhx = hex::encode(&node_id);
        // println!("node id = {}", nidhx);

        // node name
        let sec = ini_section(ini, "node");

        let defnm: String = "hn".to_owned() + &nidhx[..8];
        let mut node_name = ini_must_maxlen(&sec, "name", &defnm, 16); // max len = 16
        // println!("node name = {}", node_name);

        // create config
        let mut cnf = NodeConf{
            node_id: node_id,
            node_name: node_name,
            boot_nodes: Vec::new(),
        };

        // ok
        cnf
    }

}