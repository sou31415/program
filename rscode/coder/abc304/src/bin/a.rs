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
        n:usize,a:[(String,usize);n],
    }
    let mut idx: usize = 0;
    let mut c: usize = std::usize::MAX;
    for i in 0..n {
        if a[i].1 < c {
            idx = i;
            c = a[i].1;
        }
    }
    for i in idx..(idx + n) {
        if i > n - 1 {
            println!("{}", a[i - n].0);
        } else {
            println!("{}", a[i].0);
        }
    }
}

pub fn ziparam(a: usize, b: usize) -> usize {
    return max(a, b) - min(a, b);
}
