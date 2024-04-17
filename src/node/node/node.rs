


pub struct HacashNode {
    cnf: NodeConf,
}


impl HacashNode {

    pub fn open(ini: &IniObj) -> HacashNode {
        let mut cnf = NodeConf::new(ini);
        HacashNode{
            cnf: cnf,
        }
    }

}