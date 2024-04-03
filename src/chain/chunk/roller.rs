

pub struct StateRoller {
    
    pub state: Weak<ChainState>, // current latest state
    pub scusp: Weak<RollChunk>, // current latest block

    pub sroot: Arc<RollChunk>, // tree root block

}


impl StateRoller {

    pub fn create(){}

}