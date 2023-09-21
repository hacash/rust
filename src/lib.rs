#![no_main]
// #![no_std]

// #[panic_handler]
// fn handle_panic(_: &core::panic::PanicInfo) -> ! {
//     loop {}
// }

#[allow(unused_macros)]
macro_rules! panic {
    ($name:expr) => {
        loop {}
    };
}


/********* mod ********/


pub mod sdk;



