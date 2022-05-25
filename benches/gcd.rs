#![feature(test)]

use std::vec;
extern crate test;

#[bench]
fn bench_gcd1(b: &mut test::Bencher) {
    let v = vec![131071u64, 43987, 31537, 2309, 534, 233, 23, 524287, 100000007];
    b.iter(|| v.iter().fold(0, |a,b | algorithm::gcd::gcd(a, *b)))
}
#[bench]
fn bench_gcd2(b: &mut test::Bencher) {
    let v = vec![131071u64, 43987, 31537, 2309, 534, 233, 23, 524287, 100000007];
    b.iter(|| v.iter().fold(0, |a,b | algorithm::gcd::_gcd1(a, *b)))
}
#[bench]
fn bench_gcd3(b: &mut test::Bencher) {
    let v = vec![131071u64, 43987, 31537, 2309, 534, 233, 23, 524287, 100000007];
    b.iter(|| v.iter().fold(0, |a,b | algorithm::gcd::_gcd2(a, *b)))
}
