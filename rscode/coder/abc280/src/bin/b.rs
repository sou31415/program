#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars, marker::Usize1};
#[allow(unused_imports)]
use std::collections::{HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        n:usize,a:[i128;n],
    }
    let mut result: Vec<i128> = vec![];
    for i in 0..n {
        if i == 0 {
            result.push(a[0]);
        } else {
            let decoy: i128 = a[i] - a[i - 1];
            result.push(decoy);
        }
        print!("{} ", result[i]);
    }
}
