#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars, marker::Usize1};
#[allow(unused_imports)]
use std::collections::HashSet;
fn main() {
    input! {
        n:usize,mut a:[usize;5*n],
    }
    a.sort();
    let mut result: f64 = 0.000000;
    let ch = &a[n..(a.len() - n)];
    for i in ch.iter() {
        result += *i as f64;
    }

    println!("{}", result / ch.len() as f64);
}
