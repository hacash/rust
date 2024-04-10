




pub fn main_test8327459283() {


    let hx = Hash::from_hex(b"faa1025aac192976049a91af8d552ebf5864f06732594b08456f6c7ab7d9a3e6");

    let srrstr: RetErr = errf!(" hash is {}", hx);
    println!("{}", srrstr.err().unwrap());



    let act1 = crate::protocol::action::ACTION_KIND_1;
    println!("{}", act1);

    let mut actobj = crate::protocol::action::HacTransfer::new();
    println!("{}", hex::encode(actobj.serialize()));




}





pub fn main_test134234() {


    let addrhac = AddrHac::new();

    println!("{} {} {}", "abc 123", s!("error"), addrhac.amount);

    let rshx = x16rs::x16rs_hash(1, &x16rs::calculate_hash(b"123456"));

    println!("{}", hex::encode(rshx));


    let pubkey = hex::decode("817ED5FC625752CBF027A39573E5F40FAC124AC1D983DD91C477C58F2A3BF983F4").unwrap();
    println!("{}", Account::to_readable(&Account::get_address_by_public_key(pubkey.try_into().unwrap())));

    

    // panic_never_call_this!();
}
