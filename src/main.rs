// mod core;


use crate::core::field;
use crate::core::interface::field::*;

fn main() {

    let mut bt = field::BytesFixed4::new();

    let data = vec![1u8,1,1,1];

    let res = bt.parse(&data, 0);

    println!("{}", res.unwrap());

    println!("{}", bt.to_hex());

}


