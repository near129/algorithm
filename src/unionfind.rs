use cargo_snippet::snippet;
#[snippet("UnionFind")]
pub struct UnionFind(Vec<i64>);

#[snippet("UnionFind")]
impl UnionFind {
    pub fn new(n: usize) -> Self {
        UnionFind(vec![-1; n])
    }
    pub fn from_pair(size: usize, pair: &[(usize, usize)]) -> Self {
        let mut uf = UnionFind::new(size);
        pair.iter().for_each(|&(a, b)| {
            uf.union(a, b);
        });
        uf
    }

    pub fn find(&mut self, x: usize) -> usize {
        if self.0[x] < 0 {
            x
        } else {
            self.0[x] = self.find(self.0[x] as usize) as i64;
            self.0[x] as usize
        }
    }

    pub fn union(&mut self, x: usize, y: usize) -> bool {
        let mut x = self.find(x);
        let mut y = self.find(y);
        if x == y {
            false
        } else {
            if self.0[x] > self.0[y] {
                std::mem::swap(&mut x, &mut y);
            }
            self.0[x] += self.0[y];
            self.0[y] = x as i64;
            true
        }
    }
    pub fn size(&mut self, x: usize) -> usize {
        let x = self.find(x);
        -self.0[x] as usize
    }
    pub fn same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }
}
