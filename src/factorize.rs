use cargo_snippet::snippet;
#[snippet]
pub fn factorize(mut n: u64) -> Vec<(u64, u32)> {
    let mut res = vec![];
    for p in 2..((n as f64).sqrt() as u64 + 1) {
        if n % p == 0 {
            let mut i = 0;
            while n % p == 0 {
                n /= p;
                i += 1;
            }
            res.push((p, i));
        }
    }
    if n != 1 {
        res.push((n, 1));
    }
    res
}
#[snippet]
pub fn divisor(n: u64) -> Vec<u64> {
    let mut res = vec![];
    for p in 1..((n as f64).sqrt() as u64 + 1) {
        if n % p == 0 {
            res.push(p);
            res.push(n/p);
        }
    }
    res.sort_unstable();
    res.dedup();
    res
}

#[cfg(test)]
mod tests {
    use super::{divisor, factorize};

    #[test]
    fn test_factorize() {
        assert_eq!(factorize(1), vec![]);
        assert_eq!(factorize(2), vec![(2, 1)]);
        assert_eq!(factorize(3), vec![(3, 1)]);
        assert_eq!(factorize(4), vec![(2, 2)]);
        assert_eq!(factorize(5), vec![(5, 1)]);
        assert_eq!(factorize(6), vec![(2, 1), (3, 1)]);
    }
    #[test]
    fn test_divisor() {
        assert_eq!(divisor(1), vec![1]);
        assert_eq!(divisor(2), vec![1, 2]);
        assert_eq!(divisor(3), vec![1, 3]);
        assert_eq!(divisor(4), vec![1, 2, 4]);
    }
}