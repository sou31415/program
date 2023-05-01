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
        n:usize,a:usize,b:usize,
        c:[usize;n],
    }
    for (i, &k) in c.iter().enumerate() {
        if k == a + b {
            println!("{}", i + 1);
        }
    }
}
