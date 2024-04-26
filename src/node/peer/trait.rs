

pub trait PeerManage: Send + Sync + dyn_clone::DynClone  {
    fn switch_peer(&self, _: Arc<Peer>) -> Arc<Peer>;
}


dyn_clone::clone_trait_object!(PeerManage);


