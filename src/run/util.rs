
macro_rules! delay_continue_ms {
    ($ms: expr) => {
        sleep(Duration::from_millis($ms));
        continue
    }
}

macro_rules! delay_continue {
    ($sec: expr) => {
        sleep(Duration::from_secs($sec));
        continue 
    }
}

macro_rules! delay_return_ms {
    ($ms: expr) => {
        sleep(Duration::from_millis($ms));
        return
    }
}

macro_rules! delay_return {
    ($sec: expr) => {
        sleep(Duration::from_secs($sec));
        return
    }
}


// 
fn hash_more_power(dst: &[u8], src: &[u8]) -> bool {
    let mut ln = dst.len();
    let l2 = src.len();
    if l2 < ln {
        ln = l2;
    }
    for i in 0..ln {
        let (l, r) = (dst[i], src[i]);
        if l < r {
            return true
        }else if l > r {
            return false
        }
    }
    return false
}

//
fn hash_left_zero_pad(dst: &[u8], pad: usize) -> Vec<u8> {
    let mut idx = 0usize;
    for i in 0 .. dst.len() {
        if dst[i] > 0 {
            idx = i;
            break
        }
    }
    dst[0 .. idx + 3].to_vec()
}

fn hash_left_zero_pad3(dst: &[u8]) -> Vec<u8> {
    hash_left_zero_pad(dst, 3)
}
