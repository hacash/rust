use std::time::Instant;


fn tdat() -> Box<dyn Action> {
    let obj = protocol::action::HacTransfer::default();
    Box::new(obj)
}

pub fn main_test28374659823746892() {


    let insv1 = "{p:hpfp,t:LONG,m:10k,do:mint,n:31}".to_owned();
    let insv2 = "{p:hart,i:hacdfun,m:200,do:bind,n:15,h:fd82b3a0234ba4b3}".to_owned();

    println!("{} {}", insv1.len(), insv2.len());

    for i in 0..=255u32 {
        println!("{}: {}", i, i.pow(3)  )
    }

}

pub fn main_test736428456983476824() {

    /*

    let mut list = DiamondNameListMax200::default();
    list.push(DiamondName::from_readable(b"AAABBB"));
    list.push(DiamondName::from_readable(b"WWWTTT"));
    list.push(DiamondName::from_readable(b"HYYNTW"));

    println!("{}", list.readable());

    let mut form = DiamondOwnedForm::default();
    form.push(&list);
    form.drop_one(&DiamondName::from_readable(b"WWWTTT"));

    println!("{}", form.readable());

    */

    /*
    let mut hxds = [255u8; 32];
    for i in 0..8 {
        for x in 1..=255 {
            hxds[i] -= 1;
            println!("{} - {}", hex::encode(&hxds), crate::mint::difficulty::hash_to_rateshow(&hxds, 300));
        }
    }
    */

    /*
    let mut numv = 0u64;
    let step = 125;
    loop {
        let av = AutoU64::from(numv);
        let avbts = av.to_bytes();
        let a2 = AutoU64::from_bytes(&avbts);
        if av != a2 || avbts != a2.to_bytes() || a2 != numv {
            println!("{} - {} - {} - {} - {}", numv, av, hex::encode(&avbts), a2, hex::encode(a2.to_bytes()));
            break
        }
        // next
        if numv + step >= u64::MAX {
            println!("{} - {} - {} - {} - {}", numv, av, hex::encode(&avbts), a2, hex::encode(a2.to_bytes()));
            break
        }
        numv += step;
        if numv / step % 1000000 == 0 {
            println!("{}", numv);
        }
        // if numv >= 200 + 16*256*256*256  {
        //     break
        // }
    }
    */

    let numvs = [
        AUTOU64XLIST[0] - 1,
        AUTOU64XLIST[0] - 0,
        AUTOU64XLIST[0] + 100,
        AUTOU64XLIST[1] - 1,
        AUTOU64XLIST[1] - 0,
        AUTOU64XLIST[1] + 100,
        AUTOU64XLIST[2] - 1,
        AUTOU64XLIST[2] - 0,
        AUTOU64XLIST[2] + 100,
        AUTOU64XLIST[3] - 1,
        AUTOU64XLIST[3] - 0,
        AUTOU64XLIST[3] + 100,
        AUTOU64XLIST[4] - 1,
        AUTOU64XLIST[4] - 0,
        AUTOU64XLIST[4] + 100,
        AUTOU64XLIST[5] - 1,
        AUTOU64XLIST[5] - 0,
        AUTOU64XLIST[5] + 100,
        AUTOU64XLIST[6] - 1,
        AUTOU64XLIST[6] - 0,
        AUTOU64XLIST[6] + 100,
        AUTOU64XLIST[7] - 1,
        AUTOU64XLIST[7] - 0,
        AUTOU64XLIST[7] + 100,
        1844_67440737_09551515,
        1844_67440737_09551615,
    ];
    for numv in numvs {
        let av = AutoU64::from(numv);
        let avbts = av.to_bytes();
        let a2 = AutoU64::from_bytes(&avbts);
        println!("{} - {} - {} - {} - {}", numv, av, hex::encode(&avbts), a2, hex::encode(a2.to_bytes()));
        if av != a2 || avbts != a2.to_bytes() || a2 != numv {
            panic_never_call_this!();
        }
    }

}

pub fn main_test_vecspeed387425983() {

    let mut datas = vec![];
    let max = 5000usize;

    for i in 0 .. 111 {
        // datas.push(tdat());
        datas.push(1u128);
    }

    let start_time = Instant::now();


    for i in 0..max {
        // datas.insert(1, tdat());
        datas.insert(1, 1u128);
    }

    for i in 0..max {
        datas.remove(1);
    }

    let end_time = Instant::now();

    let elapsed_time = end_time - start_time;
    println!("code run time: {:?}", elapsed_time);

    // println!("vec len {} {} ", datas.len(), hex::encode(datas[0].serialize()));
    println!("vec len {}", datas.len());


}


pub fn main_test8327459283() {


    let hx = Hash::from_hex(b"faa1025aac192976049a91af8d552ebf5864f06732594b08456f6c7ab7d9a3e6");

    let srrstr: RetErr = errf!(" hash is {}", hx);
    println!("{}", srrstr.err().unwrap());



    let act1 = crate::protocol::action::ACTION_KIND_1;
    println!("{}", act1);

    let mut actobj = crate::protocol::action::HacTransfer::default();
    println!("{}", hex::encode(actobj.serialize()));




}





pub fn main_test134234() {


    let addrhac = AddrHac::default();

    println!("{} {} {}", "abc 123", s!("error"), addrhac.amount);

    let rshx = x16rs::x16rs_hash(1, &x16rs::calculate_hash(b"123456"));

    println!("{}", hex::encode(rshx));


    let pubkey = hex::decode("817ED5FC625752CBF027A39573E5F40FAC124AC1D983DD91C477C58F2A3BF983F4").unwrap();
    println!("{}", Account::to_readable(&Account::get_address_by_public_key(pubkey.try_into().unwrap())));

    

    // panic_never_call_this!();
}
