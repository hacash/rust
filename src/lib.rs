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

/********* mod ********/



/********* mod ********/

#[macro_use]
mod sys;
#[macro_use]
mod base;
mod interface;
mod core;



