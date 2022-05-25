#[macro_export]
macro_rules! join {
    ($iter:expr) => {
        $iter.map(|x| x.to_string()).collect::<Vec<_>>().join(" ")
    };
    ($iter:expr, $sep: expr) => {
        $iter.map(|x| x.to_string()).collect::<Vec<_>>().join($sep)
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn join() {
        let string = vec!["a", "b", "c"];
        let number = vec![1, 2, 3];
        assert_eq!(join!(string.iter()), "a b c");
        assert_eq!(join!(number.iter()), "1 2 3");
        assert_eq!(join!(string.iter(), "\n"), "a\nb\nc");
        assert_eq!(join!(number.iter(), "\n"), "1\n2\n3");
    }
}
