#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars, marker::Usize1};
#[allow(unused_imports)]
use std::collections::{HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        a:usize,x:usize,mut m:usize,
    }
    let mut mat: Vec<Vec<usize>> = vec![vec![a, 1], vec![0, 1]];
    let result = f(&mut mat, m, x - 1);
    println!("{}", result);
}

fn f(v: &mut Vec<Vec<usize>>, m: usize, n: usize) -> usize {
    let decoy: usize = v[0][1] % m;
    decoy
}
