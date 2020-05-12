#![feature(test)]

extern crate escaper;

extern crate rand;
extern crate test;

use escaper::*;

static BIG_STR: &'static str = include_str!("../moonstone-short.txt");

#[bench]
fn bench_encode_attribute(bh: &mut test::Bencher) {
    bh.iter(|| encode_attribute(BIG_STR));
    bh.bytes = BIG_STR.len() as u64;
}

#[bench]
fn bench_encode_minimal(bh: &mut test::Bencher) {
    bh.iter(|| encode_minimal(BIG_STR));
    bh.bytes = BIG_STR.len() as u64;
}

#[bench]
fn bench_decode_attribute(bh: &mut test::Bencher) {
    let encoded = encode_attribute(BIG_STR);
    bh.iter(|| decode_html(&encoded));
    bh.bytes = encoded.len() as u64;
}

#[bench]
fn bench_decode_minimal(bh: &mut test::Bencher) {
    let encoded = encode_minimal(BIG_STR);
    bh.iter(|| decode_html(&encoded));
    bh.bytes = encoded.len() as u64;
}
