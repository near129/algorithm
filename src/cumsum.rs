use std::ops::AddAssign;

pub fn cumsum1<T: AddAssign+Copy>(v: &[T], init: T) -> Vec<T> {
    [init].iter()
       .chain(v.iter())
       .scan(init, |sum, &x| {
           *sum += x;
           Some(*sum)
       }).collect::<Vec<T>>()
}