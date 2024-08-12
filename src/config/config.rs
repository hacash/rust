

// pub type IniObj = HashMap<String, HashMap<String, Option<String>>>;


/**
 * get datadir
 */
 fn get_datadir(ini: &IniObj) -> String {

    let sec = ini_section(ini, "default"); // default = root
    let mut data_dir = ini_must(&sec, "data_dir", "hacash_mainnet_data");

    let mut ddrp = PathBuf::from(&data_dir);
    // println!("{:?} {}", ddrp, ddrp.is_absolute());
    if ! ddrp.is_absolute() {
        let mut dtdir = std::env::current_exe().unwrap().parent().unwrap().to_path_buf().join(&data_dir);
        data_dir = dtdir.to_str().unwrap().to_string();
        if dtdir.exists() {
            data_dir = dtdir.canonicalize().unwrap().to_str().unwrap().to_string();
        }
    }

    data_dir
}


/**
 * get data path
 */
 fn get_data_path(ini: &IniObj) -> PathBuf {

    let sec = ini_section(ini, "default"); // default = root
    let mut data_dir = ini_must(&sec, "data_dir", "hacash_mainnet_data");

    let mut ddrp = PathBuf::from(&data_dir);
    // println!("{:?} {}", ddrp, ddrp.is_absolute());
    if ! ddrp.is_absolute() {
        ddrp = std::env::current_exe().unwrap().parent().unwrap().to_path_buf().join(&data_dir);
    }
    ddrp
}


/**
 * load config
 */
pub fn load_config(mut cnfilestr: String) -> IniObj {

    // let mut cnfilestr = "./hacash.config.ini".to_string();

    // exec dir
    let mut execdir = std::env::current_exe().unwrap().parent().unwrap().to_path_buf();
    let mut cnf_file = execdir.join(&cnfilestr);

    // cmd args
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        cnfilestr = args[1].clone();
        cnf_file = PathBuf::from(&cnfilestr);
    }

    // check exists
    if ! cnf_file.exists() {
        // error
        println!("[Config Error] Cannot find config file {}", cnfilestr);
        return IniObj::new()
    }

    cnfilestr = cnf_file.canonicalize().unwrap().to_str().unwrap().to_string();
    // println!("{:?} {:?}", args, exedir);
    println!("[Config] Load: {} {}.", cnfilestr, sys::ctshow());
    
    // load file
    ini!(&cnfilestr)

}
