

#[no_mangle]
pub extern fn trs_test(x: i32) -> i32 {
    let mut bts = vec![1,0,5,1,1,1,1,1,1,1];
    bts[1] = x;
    if x > 100 {
        panic!("error more 100")
    }
    let mut res = 0;
    for v in bts {
        res += v;
    }
    res + 10
}


