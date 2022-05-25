#![feature(test)]

extern crate test;

#[bench]
fn bench_prime1(b: &mut test::Bencher) {
    b.iter(|| algorithm::prime::prime_table(10usize.pow(7)))
}
#[bench]
fn bench_prime2(b: &mut test::Bencher) {
    b.iter(|| algorithm::prime::_prime_table(10usize.pow(7)))
}