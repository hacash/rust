
pub const HASHRATE_VALUE_BASE: [u8; HASH_WIDTH] = [0,0,0,0,0,0,0,0,
    255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255,255];

const VK: u128 = 1000; 
const VM: u128 = VK * VK;
const VG: u128 = VM * VK;
const VT: u128 = VG * VK;
const VP: u128 = VT * VK;
const VE: u128 = VP * VK;
const VZ: u128 = VE * VK;
const VY: u128 = VZ * VK;
const VB: u128 = VY * VK;

const HNS: [&str; 9] = ["K","M","G","T","P","E","Z","Y","B"];
const HVS: [u128; 9] = [VK, VM, VG, VT, VP, VE, VZ, VY, VB];


pub fn rates_to_show(rates: u128) -> String {
    let mut hsx = HVS.len() - 1;
    for i in 0..HVS.len() {
        if rates / HVS[i] < VK {
            hsx = i;
            break
        }
    }
    let num = (rates as f64) / (HVS[hsx] as f64);
    let unit = HNS[hsx];
    format!("{:.2}{}c/s", num, unit)
}

pub fn hash_to_rateshow(hx: &[u8; HASH_WIDTH], secs: u64) -> String {
    let rates = hash_to_rates(hx, secs);
    rates_to_show(rates)
}

pub fn u32_to_rateshow(num: u32, secs: u64) -> String {
    let rates = u32_to_rates(num, secs);
    rates_to_show(rates)
}

pub fn u32_to_rates(num: u32, secs: u64) -> u128 {
    let hx = u32_to_hash(num);
    hash_to_rates(&hx, secs)
}

pub fn hash_to_rates(hx: &[u8; HASH_WIDTH], secs: u64) -> u128 {
    hash_to_power(&hx) / (secs as u128)
}

pub fn hash_to_power(hx: &[u8; HASH_WIDTH]) -> u128 {
    let bigv = BigInt::from_bytes_be(BigSign::Plus, &hx[..]);
    let base = BigInt::from_bytes_be(BigSign::Plus, &HASHRATE_VALUE_BASE[..]);
    let power = base.to_f64().unwrap() / bigv.to_f64().unwrap() * (u64::MAX as f64 + 1.0);
    power as u128
}






