//submitted abc293_c
#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars, marker::Usize1};
#[allow(unused_imports)]
use std::collections::{HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        n:usize , m:usize,
        a:[usize;n],
        b:[usize;m],
    }
    let mut v: Vec<usize> = vec![];
    for i in 0..n {
        v.push(a[i]);
    }
    for i in 0..m {
        v.push(b[i]);
    }
    v.sort();
    for i in 0..a.len() {
        print!("{} ", v.binary_search(&a[i]).unwrap() + 1);
    }
    println!("");
    for i in 0..b.len() {
        print!("{} ", v.binary_search(&b[i]).unwrap() + 1);
    }
}
