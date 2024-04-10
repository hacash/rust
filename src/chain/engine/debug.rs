


impl BlockEngine {

    pub fn print_roller(&self) {
        let roller = self.klctx.lock().unwrap();
        _print_one(0, roller.sroot.clone());
    }

}

fn _print_one(tab: usize, rl: Arc<RollChunk>) {
    let indentation = "  ".repeat(tab);
    let tagstr = rl.print();
    let childs = rl.childs.borrow();
    if childs.len() == 0 {
        println!("{}<{} />", indentation, tagstr);
        return
    }
    println!("{}<{}>", indentation, tagstr);
    for li in childs.iter() {
        _print_one(tab+1, li.clone());
    }
    println!("{}</{}>", indentation, tagstr);
}
