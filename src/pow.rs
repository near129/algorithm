use cargo_snippet::snippet;
#[snippet]
pub fn pow(mut a: u64, mut b: u64, m: u64) -> u64 {
    a %= m;
    let mut ret = 1;
    while b > 0 {
        if b & 1 == 1 {
            ret = (ret * a) % m;
        }
        a = (a * a) % m;
        b >>= 1;
    }
    ret
}