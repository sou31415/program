#![allow(unused_imports)]
use proconio::{fastout, input, marker::Usize1};
use std::collections::HashSet;
#[fastout]
fn main() {
    input! {
        n:usize,q:usize,
        c:[Usize1;n],
    }
    let size = n / rt(q) + 1;
    let mut query: Vec<Vec<(usize, usize, usize)>> = vec![vec![]; rt(q) + 1];
    for i in 0..q {
        input! {
            l:Usize1,r:Usize1,
        }
        query[l / size].push((i, l, r));
    }
    let mut result: Vec<usize> = vec![0; q];
    let mut left: usize = 0;
    let mut right: usize = 0;
    let mut num: Vec<usize> = vec![0; n];
}

pub fn rt(n: usize) -> usize {
    let mut l: u128 = 1;
    let mut r: u128 = n as u128;
    let mut m: u128 = 0;
    while r - l > 1 {
        m = (r + l) / 2;
        if m * m == n as u128 {
            return m as usize;
        }
        if m * m > n as u128 {
            r = m;
        } else {
            l = m;
        }
    }
    return l as usize;
}
