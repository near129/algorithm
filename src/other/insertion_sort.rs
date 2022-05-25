use std::fmt::Display;


pub fn insertion_sort<T: Ord + Display + Copy>(a: &mut Vec<T>) {
    for mut i in 1..a.len() {
        let v = a[i];
        while i > 0 && a[i-1] > v {
            a[i] = a[i-1];
            i -= 1;
        }
        a[i] = v;
        // for debug
        println!("{}", a.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
    }
}
pub trait InsertSort {
    fn insertion_sort(&mut self);
}
impl<T> InsertSort for Vec<T> where
    T: Ord + Display + Copy{
        fn insertion_sort(&mut self) {
        for mut i in 1..self.len() {
            let v = self[i];
            while i > 0 && self[i-1] > v {
                self[i] = self[i-1];
                i -= 1;
            }
            self[i] = v;
            // for debug
            println!("{}", self.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "));
        }
    }
}
#[cfg(test)]
mod test {
    use std::vec;

    use super::*;
    #[test]
    fn test_sort() {
        let v = vec![
            vec![1, 2, 3, 4, 5],
            vec![5, 4, 3, 2, 1],
            vec![1, 1, 1, 1, 1],
            vec![4, 3, 5, 1, 1]
        ];
        for mut vv in v {
            let mut a = vv.clone();
            vv.sort_unstable();
            a.insertion_sort();
            assert_eq!(a, vv);
        }
    }
}