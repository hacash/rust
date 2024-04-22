


impl BlockEngine {

    pub fn print_roller(&self) {
        let roller = self.klctx.lock().unwrap();
        // print tree
        _print_one(0, roller.sroot.clone());
        // print head
        let mut prev = roller.scusp.clone();
        let mut paths = vec![];
        for i in 0..10 {
            if let Some(p) = prev.upgrade() {
                paths.push(p.print());
                prev = p.parent.clone();
            }else{
                break;
            }
        }
        paths.reverse();
        println!("[ {} ]", paths.join(" -> "));
    }

}


// print one
fn _print_one(tab: usize, rl: Arc<RollChunk>) {
    let indentation = "  ".repeat(tab);
    let tagstr = rl.print();
    let childs = rl.childs.lock().unwrap();
    if childs.len() == 0 {
        println!("{}<{} />", indentation, tagstr);
        return
    }
    println!("{}<{} c={}>", indentation, tagstr, childs.len());
    for li in childs.iter() {
        _print_one(tab+1, li.clone());
    }
    println!("{}</{}>", indentation, tagstr.split(":").collect::<Vec<_>>()[0]);
}
