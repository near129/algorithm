use cargo_snippet::snippet;
#[snippet("Combination")]
pub struct Combination {
    fact: Vec<usize>,
    fact_inv: Vec<usize>,
    modulo: usize,
}

#[snippet("Combination")]
impl Combination {
    pub fn new(max: usize, modulo: usize) -> Self {
        let mut fact = vec![1; max + 1];
        let mut fact_inv = vec![1; max + 1];
        let mut inv = vec![1; max + 1];
        for i in 2..max + 1 {
            fact[i] = fact[i - 1] * i % modulo;
            inv[i] = modulo - inv[modulo % i] * (modulo / i) % modulo;
            fact_inv[i] = fact_inv[i - 1] * inv[i] % modulo;
        }
        Self {
            fact,
            fact_inv,
            modulo,
        }
    }
    pub fn calc(&self, n: usize, k: usize) -> usize {
        if n < k {
            0
        } else {
            self.fact[n] * (self.fact_inv[k] * self.fact_inv[n - k] % self.modulo) % self.modulo
        }
    }
}
#[cfg(test)]
mod test {
    use super::*;

    fn gcd(a: usize, b: usize) -> usize {
        if b == 0 {
            a
        } else {
            gcd(b, a % b)
        }
    }

    #[test]
    fn random_combination() {
        let modulo = 1_000_000_007;

        for n in 100..200 {
            let comb = Combination::new(n, modulo);
            for m in 0..(n + 1) {
                let mut upper = (0..m).map(|i| n - i).collect::<Vec<_>>();
                for i in 0..m {
                    let mut divisor = i + 1;
                    for u in upper.iter_mut().take(i + 1) {
                        if divisor == 1 {
                            break;
                        }

                        let g = gcd(divisor, *u);
                        *u /= g;
                        divisor /= g;
                    }
                }

                let mut check = 1;
                for u in &upper {
                    check = (check * u) % modulo;
                }

                assert_eq!(comb.calc(n, m), check);
            }
        }
    }
}
