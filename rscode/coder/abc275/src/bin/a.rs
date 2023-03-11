#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars, marker::Usize1};
#[allow(unused_imports)]
use std::collections::{HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        n:usize , a:[usize;n],
    }
    let mut d: Vec<usize> = a.clone();
    d.sort();
    for i in 0..n {
        if a[i] == d[n - 1] {
            println!("{}", i + 1);
            return;
        }
    }
}
