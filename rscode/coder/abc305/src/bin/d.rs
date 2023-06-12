#![allow(unused_imports)]
use itertools::Itertools;
use petgraph::unionfind::UnionFind;
use proconio::{fastout, input, marker::Chars, marker::Usize1};
use std::cmp::{max, min};
use std::collections::{BTreeSet, HashSet, VecDeque};
#[fastout]
fn main() {
    input! {
        n:usize,a:[usize;n],
        q:usize,
        lr:[(usize,usize);q],
    }
    let mut v: Vec<usize> = vec![0; n];
    for i in 1..(n - 1) {
        if i % 2 == 1 {
            v[i] = v[i - 1] + a[i] - a[i - 1];
        } else {
            v[i] = v[i - 1];
        }
    }
}

pub fn ziparam(a: usize, b: usize) -> usize {
    return max(a, b) - min(a, b);
}
