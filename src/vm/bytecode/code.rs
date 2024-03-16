
macro_rules! define_bytecode_ptrs {
    ($( $name:ident : $bytv:expr)+) => {  
        $(
concat_idents!(op_name = OPC_, $name { pub const op_name: u8 =  $bytv; });
        )+
    }
}

// define
// pub const OPC_NOP: u8 = 0xfd;
define_bytecode_ptrs!{



    END   : 0xff // end / func return
    ABT   : 0xfe // abort / fatal end
    NOP   : 0xfd // do nothing



}


