

pub struct StateBrick {

    height: BlockHeight, 
    block: BlockPkg,
    state: Rc<ChainState>,

    childs: Vec<Rc<StateBrick>>,
    parent: Weak<StateBrick>,

}


