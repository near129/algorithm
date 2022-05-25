use std::mem::swap;
use cargo_snippet::snippet;


#[snippet]
pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    loop {
        if a == 0 {
            return b;
        } else {
            b %= a;
            swap(&mut a, &mut b);
        }
    }
}
#[snippet(include="gcd")]
pub fn lcm(a: u64, b: u64) -> u64 {
     a*b / gcd(a, b)
}
pub fn _gcd1(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let tmp = a % b;
        a = b;
        b = tmp;
    }
    a
}
pub fn _gcd2(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}
#[cfg(test)]
mod tests {
    use super::gcd;

    #[test]
    fn test_gcd_zero() {
        for i in 0..20 {
            assert_eq!(gcd(0, i), i);
            assert_eq!(gcd(i, 0), i);
        }
    }

    #[test]
    fn test_gcd_one() {
        for i in 0..20 {
            assert_eq!(gcd(1, i), 1);
            assert_eq!(gcd(i, 1), 1);
        }
    }

    #[test]
    fn test_gcd_identical() {
        for i in 0..20 {
            assert_eq!(gcd(i, i), i);
        }
    }

    #[test]
    fn test_gcd_divisible() {
        for i in 0..20 {
            for j in 0..20 {
                assert_eq!(gcd(i, i * j), i);
                assert_eq!(gcd(i * j, i), i);
            }
        }
    }
}
