#![allow(unused_imports)]
use itertools::Itertools;
use petgraph::unionfind::UnionFind;
use proconio::{fastout, input, marker::Chars, marker::Usize1, source::line::LineSource};
use std::cmp::{max, min, Reverse};
use std::collections::{BTreeSet, HashSet, VecDeque};
use std::io::{stdin, stdout, BufReader};
use superslice::Ext;
#[fastout]
fn main() {
    input! {
        a:isize,m:usize,l:isize,r:isize,
    }
    let d = r - l;
    let d_ans = d / (m as isize);
    if is_tree(a, m, l) && is_tree(a, m, r) {
        println!("{}", d_ans + 1);
    } else {
        println!("{}", d_ans);
    }
}
pub fn ziparam(a: usize, b: usize) -> usize {
    // |a:usize - b:usize| -> usize
    return max(a, b) - min(a, b);
}
fn is_tree(a: isize, m: usize, p: isize) -> bool {
    if p - a % m as isize == 0 {
        true
    } else {
        false
    }
}
