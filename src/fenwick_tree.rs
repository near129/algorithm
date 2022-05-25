#[allow(dead_code)]
struct FenwickTree<T> {
    // http://hos.ac/slides/20140319_bit.pdf
    data: Vec<T>,
    init: T,
}
#[allow(dead_code)]
impl<T: Copy + std::ops::AddAssign + std::ops::Sub<Output = T>> FenwickTree<T> {
    fn new(length: usize, init: T) -> FenwickTree<T> {
        FenwickTree {
            data: vec![init; length],
            init,
        }
    }
    fn add(&mut self, mut index: usize, value: T) {
        while index < self.data.len() {
            self.data[index] += value;
            index |= index + 1;
        }
    }
    fn accum(&self, index: usize) -> T {
        let mut res = self.init;
        let mut index = index as i32 - 1;
        while index >= 0 {
            res += self.data[index as usize];
            index = (index & (index + 1)) - 1
        }
        res
    }
    fn sum(&self, l: usize, r: usize) -> T {
        self.accum(r) - self.accum(l)
    }
}
#[cfg(test)]
mod test {
    use super::*;
    use rand::{thread_rng, Rng};

    #[test]
    fn random_array() {
        let n = 1000;
        let mut bit = FenwickTree::new(n, 0);
        let mut v = vec![0; n];

        for _ in 0..10000 {
            let value = thread_rng().gen_range(0, 1000);
            let k = thread_rng().gen_range(0, n);
            v[k] += value;
            bit.add(k, value);

            let mut sum = 0;
            for (i, vv) in v.iter().enumerate() {
                sum += vv;
                assert_eq!(sum, bit.sum(0, i + 1));
            }
        }
    }
}
