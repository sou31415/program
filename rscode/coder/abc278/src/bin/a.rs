#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars, marker::Usize1};
#[allow(unused_imports)]
use std::collections::{HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        n:usize , k:usize,
        mut a:[usize;n],
    }
    for _ in 0..k {
        a.reverse();
        a.pop();
        a.reverse();
        a.push(0);
    }
    for i in 0..n {
        print!("{} ", a[i]);
    }
}
