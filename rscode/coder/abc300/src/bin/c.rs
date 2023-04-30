#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars, marker::Usize1};
#[allow(unused_imports)]
use std::collections::{BTreeSet, HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        h:usize,w:usize,
        a:[Chars;h],
    }
    let mut c: Vec<Vec<char>> = vec![vec!['.'; 450]; 450];
    for i in 0..h {
        for j in 0..w {
            c[150 + i][150 + j] = a[i][j];
        }
    }
    let mut m: usize = std::cmp::min(h, w);
}
