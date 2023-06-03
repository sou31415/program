#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars, marker::Usize1};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{BTreeSet, HashSet, VecDeque};
#[fastout]
fn main() {
    input! {
        n:usize,
    }
    let v = n
        .to_string()
        .chars()
        .map(|x| x.to_string().parse::<usize>().unwrap())
        .collect::<Vec<usize>>();
    if n < 100 {
        println!("{}", n);
        return;
    }
    for i in 0..3 {
        print!("{}", v[i]);
    }
    for i in 3..v.len() {
        print!("0");
    }
}

pub fn ziparam(a: usize, b: usize) -> usize {
    return max(a, b) - min(a, b);
}
