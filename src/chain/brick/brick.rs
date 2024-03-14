

struct StateBrick {

    block: BlockPkg,
    state: ChainState,

    childs: Vec<Box<StateBrick>>,
    parent: Weak<StateBrick>,

}


