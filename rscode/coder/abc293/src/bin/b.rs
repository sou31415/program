#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars, marker::Usize1};
#[allow(unused_imports)]
use std::collections::{HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        n:usize,a:[Usize1;n],
    }
    let mut v: Vec<bool> = vec![false; n];
    for i in 0..n {
        if !v[i] {
            v[a[i]] = true;
        }
    }
    println!("{}", v.iter().filter(|x| x == &&false).count());
    for i in 0..n {
        if !v[i] {
            print!("{} ", i + 1);
        }
    }
}
