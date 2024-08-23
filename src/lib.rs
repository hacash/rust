#![no_main]
// #![no_std]

// #[panic_handler]
// fn handle_panic(_: &core::panic::PanicInfo) -> ! {
//     loop {}
// }

#[allow(unused_macros)]
macro_rules! panic {
    ($s:expr) => {
        loop {}
    };
    ($fmt:expr, $($s:expr),+) => {
        loop {}
    };
}

#[macro_use]
extern crate lazy_static; 

/********* mod ********/

pub mod x16rs;

#[macro_use]
pub mod sys;
#[macro_use]
pub mod base;
pub mod interface;
pub mod config;
#[macro_use]
pub mod core;
#[macro_use]
pub mod protocol;
pub mod mint;
#[macro_use]
pub mod vm;
pub mod chain;
pub mod node;
pub mod server;
pub mod run;



