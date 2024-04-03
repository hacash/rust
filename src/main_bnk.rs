#[macro_use] 
mod core;
mod x16rs;


use crate::core::field::Amount;
use crate::core::field;
use crate::core::interface::field::*;

fn main() {

    let mut bt = field::Fixed4::new();

    let data = vec![1u8,1,1,1];

    let res = bt.parse(&data, 0);

    println!("{}", res.unwrap());

    println!("{}", bt.to_hex());

    println!("{}", Amount::new_small(1, 250).to_zhu_unsafe().to_string());
    // println!("{}", Amount::from_string_unsafe(&"100".to_string()).unwrap().to_zhu_unsafe().to_string());

}

