#![allow(unused_imports)]
use itertools::Itertools;
use petgraph::unionfind::UnionFind;
use proconio::{fastout, input, marker::Chars, marker::Usize1};
use std::cmp::{max, min};
use std::collections::{BTreeSet, HashSet, VecDeque};
#[fastout]
fn main() {
    input! {
        a:char,b:char,
    }
    let v: Vec<usize> = vec![3, 1, 4, 1, 5, 9];
    let mut d = (a as u8 - 'A' as u8) as usize;
    let mut c = (b as u8 - 'A' as u8) as usize;
    if d > c {
        std::mem::swap(&mut d, &mut c);
    }
    let mut result: usize = 0;
    for i in d..c {
        result += v[i];
    }
    println!("{}", result);
}

pub fn ziparam(a: usize, b: usize) -> usize {
    return max(a, b) - min(a, b);
}
