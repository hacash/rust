

pub fn merge_mrkl_root(list: &Vec<Hash>) -> Hash {
    let num = list.len();
    if num < 1 {
        panic!("merge_mrkl_root hash list cannot empty")
    }
    let mut res = vec![];
    let mut x = 0usize;
    loop {
        let lh = list[x].to_vec();
        let rh = match x+1 < num {
            true => list[x+1].to_vec(),
            false => lh.clone(),
        };
        let hx = x16rs::calculate_hash(vec![lh, rh].concat());
        res.push(Hash::must(&hx[..]));
        x += 2;
        if x >= num {
            break
        }
    }
    res[0]
}

