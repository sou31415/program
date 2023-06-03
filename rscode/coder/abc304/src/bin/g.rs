#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars, marker::Usize1};
#[allow(unused_imports)]
use std::collections::{BTreeSet, HashSet, VecDeque};
#[allow(unused_imports)]
use std::cmp::{max,min};
#[fastout]
fn main() {
    input! {
}

pub fn ziparam(a:usize,b:usize) -> usize{
    return max(a,b) - min(a,b);
}
