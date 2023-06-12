#![allow(unused_imports)]
use itertools::Itertools;
use petgraph::unionfind::UnionFind;
use proconio::{fastout, input, marker::Chars, marker::Usize1};
use std::collections::{BTreeSet, HashSet, VecDeque};
use std::cmp::{max,min};
#[fastout]
fn main() {
    input! {
}

pub fn ziparam(a:usize,b:usize) -> usize{
    return max(a,b) - min(a,b);
}
