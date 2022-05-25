use cargo_snippet::snippet;
#[allow(dead_code)]
pub fn prime_table(n: usize) -> Vec<bool> {
    let mut table = vec![true; n];
    table[0] = false;
    table[1] = false;
    for i in 2..n {
        if table[i] {
            let mut j = i + i;
            while j < n {
                table[j] = false;
                j += i
                // for j in ((i+i)..n).step_by(i) {
                //     table[j] = false;
            }
        }
    }
    table
}
#[allow(dead_code)]
pub fn _prime_table(n: usize) -> Vec<bool> {
    let mut table = vec![true; n];
    table[0] = false;
    table[1] = false;
    for i in 2..n {
        if table[i] {
            for j in ((i + i)..n).step_by(i) {
                table[j] = false;
            }
        }
    }
    table
}
#[allow(dead_code)]
pub fn prime_list(n: usize) -> Vec<usize> {
    prime_table(n)
        .iter()
        .enumerate()
        .filter(|&(_, &is_prime)| is_prime)
        .map(|(i, _)| i)
        .collect()
}
#[allow(dead_code)]
#[snippet]
pub fn is_prime(n: usize) -> bool {
    if n == 2 {
        true
    } else if n == 1 || n % 2 == 0 {
        false
    } else {
        for i in 1..((n as f64).sqrt() as usize + 1) / 2 {
            if n % (i * 2 + 1) == 0 {
                return false;
            }
        }
        true
    }
}
#[snippet]
pub struct SieveOfEratosthenes(Vec<usize>);
#[allow(dead_code)]
#[snippet]
impl SieveOfEratosthenes {
    pub fn new(n: usize) -> Self {
        let mut sieve = vec![0; n];
        sieve[0] = 1;
        for i in 2..n {
            if sieve[i] != 0 {
                continue;
            }
            sieve[i] = i;
            for j in (i + i..n).step_by(i) {
                sieve[j] = i;
            }
        }
        Self(sieve)
    }
    pub fn is_prime(&self, x: usize) -> bool {
        self.0[x] == x
    }
    pub fn get_prime_list(&self) -> Vec<usize> {
        self.0
            .iter()
            .enumerate()
            .filter(|&(i, x)| i == *x)
            .map(|(_, x)| *x)
            .collect()
    }
    pub fn factorize(&self, mut x: usize) -> Vec<usize> {
        let mut ret = vec![];
        while x > 1 {
            let p = self.0[x];
            while x % p == 0 {
                x /= p;
            }
            ret.push(p);
        }
        ret
    }
    pub fn factorize_exp(&self, mut x: usize) -> Vec<(usize, usize)> {
        let mut ret = vec![];
        while x > 1 {
            let mut exp = 0;
            let p = self.0[x];
            while x % p == 0 {
                x /= p;
                exp += 1;
            }
            ret.push((p, exp));
        }
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::{is_prime, prime_list, prime_table, SieveOfEratosthenes};
    #[test]
    fn test_primes() {
        assert_eq!(prime_list(10), vec![2, 3, 5, 7])
    }
    #[test]
    fn test_prime() {
        let tbl = prime_table(1_000_000);
        for (i, p) in tbl.iter().enumerate() {
            assert_eq!(is_prime(i), *p, "i: {}", i)
        }
    }
    #[test]
    fn sieve_of_eratosthenes_prime_list() {
        let sieve = SieveOfEratosthenes::new(10);
        assert_eq!(sieve.get_prime_list(), vec![2, 3, 5, 7])
    }
}
