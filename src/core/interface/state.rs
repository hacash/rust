use concat_idents::concat_idents;

use crate::core::Error;
use crate::core::field::*;
use crate::core::store::*;


macro_rules! define_chain_state {
    (
        ($( $kfix1:tt $name1:ident: $vtype1:ty )*),
        ($( $kfix2:tt $name2:ident: $keyty2:ty, $vtype2:ty )*)
    ) => (


pub trait ChainStateRead {

    fn pending_block_height(&self) -> BlockHeight;
    fn pending_block_hash(&self) -> Option<Hash> { None }

    $(
        concat_idents!(fn_get_1 = get_, $name1 {
            fn fn_get_1 (&self) -> Result<$vtype1, Error>;
        });  
    )*
    
    $(
        concat_idents!(fn_get_2 = get_, $name2 {
            fn fn_get_2 (&self, _: &$keyty2) -> Result<Option<$vtype2>, Error> { Ok(None) }
        });
    )*

}





pub trait ChainState : ChainStateRead {

    $(
        concat_idents!(fn_set_1 = set_, $name1 {
            fn fn_set_1(&mut self, _: &$vtype1) -> Option<Error>;
        });
        concat_idents!(fn_del_1 = del_, $name1 {
            fn fn_del_1(&mut self) -> Option<Error>;
        });
    )*

    $(
        concat_idents!(fn_set_2 = set_, $name2 {
            fn fn_set_2(&mut self, _: &$keyty2, _: &$vtype2) -> Option<Error>;
        });
        concat_idents!(fn_del_2 = del_, $name2 {
            fn fn_del_2(&mut self, _: &$keyty2) -> Option<Error>;
        });
    )*


}




    )
}



////////////////////////



define_chain_state!(
(

    // 1u8 total_supply:       TotalSupply
    // 2u8 latest_block_intro: BlockIntroSto
    // 3u8 latest_diamond:     DiamondSmeltSto
    
),(
                           
    22u8 balance: Address, BalanceSto

)
);
