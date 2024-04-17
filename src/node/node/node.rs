


pub struct HacashNode {
    cnf: NodeConf,
    engine: Arc<dyn Engine>,
}


impl HacashNode {

    pub fn open(ini: &IniObj, engine: Arc<dyn Engine>) -> HacashNode {
        let mut cnf = NodeConf::new(ini);
        HacashNode{
            cnf: cnf,
            engine: engine,
        }
    }

}