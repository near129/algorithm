use cargo_snippet::snippet;
#[snippet("BinarySearch")]
pub trait BinarySearch<T> {
    fn lower_bound(&self, x: &T) -> usize;
    fn upper_bound(&self, x: &T) -> usize;
}

#[snippet("BinarySearch")]
impl<T: Ord> BinarySearch<T> for [T] {
    fn lower_bound(&self, x: &T) -> usize {
        let mut ng = -1isize;
        let mut ok = self.len() as isize;
        while ok - ng > 1 {
            let mid = ng + (ok - ng) / 2;
            match self[mid as usize].cmp(x) {
                std::cmp::Ordering::Greater | std::cmp::Ordering::Equal => ok = mid,
                std::cmp::Ordering::Less => ng = mid,
            }
        }
        ok as usize
    }
    fn upper_bound(&self, x: &T) -> usize {
        let mut ng = -1isize;
        let mut ok = self.len() as isize;
        while ok - ng > 1 {
            let mid = ng + (ok - ng) / 2;
            match self[mid as usize].cmp(x) {
                std::cmp::Ordering::Greater => ok = mid,
                std::cmp::Ordering::Less | std::cmp::Ordering::Equal => ng = mid,
            }
        }
        ok as usize
    }
}

#[cfg(test)]
mod tests {
    use super::BinarySearch;

    #[test]
    fn test_binary_search() {
        let v1 = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(v1.lower_bound(&5), 4);
        assert_eq!(v1.upper_bound(&5), 5);
        assert_eq!(v1.lower_bound(&1), 0);
        assert_eq!(v1.upper_bound(&1), 1);
        assert_eq!(v1.lower_bound(&9), 8);
        assert_eq!(v1.upper_bound(&9), 9);
        assert_eq!(v1.lower_bound(&0), 0);
        assert_eq!(v1.upper_bound(&0), 0);
        assert_eq!(v1.lower_bound(&10), 9);
        assert_eq!(v1.upper_bound(&10), 9);
    }
    #[test]
    fn test_binary_search_same_value() {
        let v2 = vec![1, 2, 3, 4, 4, 4, 5, 6, 7];
        assert_eq!(v2.lower_bound(&4), 3);
        assert_eq!(v2.upper_bound(&4), 6);
    }
    #[test]
    fn test_binary_search_missing_value() {
        let v3 = vec![1, 2, 3, 5, 6, 7];
        assert_eq!(v3.lower_bound(&4), 3);
        assert_eq!(v3.upper_bound(&4), 3);
    }
}