




fn mrkl_merge(list: &Vec<Hash>) -> Vec<Hash> {
    let num = list.len();
    let mut res = vec![];
    let mut x = 0usize;
    loop {
        let lh = list[x].to_vec();
        let rh = match x+1 < num {
            true => list[x+1].to_vec(),
            false => lh.clone(),
        };
        let hx = x16rs::calculate_hash(vec![lh, rh].concat());
        res.push(Hash::must(&hx));
        x += 2;
        if x >= num {
            break
        }
    }
    res
}


/**
 * 
 */
pub fn calculate_mrklroot(list: &Vec<Hash>) -> Hash {
    let mut reslist = list;
    let mut tmp;
    loop {
        // println!("mrklroot len={}", list.len());
        if reslist.len() <= 1 {
            return reslist[0].clone()
        }
        tmp = mrkl_merge(&reslist);
        reslist = &tmp;
    }
}

