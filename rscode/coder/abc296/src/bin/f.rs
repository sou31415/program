#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars, marker::Usize1};
#[allow(unused_imports)]
use std::collections::{BTreeSet, HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        n:usize,
        mut a:[usize;n],
        mut b:[usize;n]
    }
    a.sort();
    b.sort();
    if a == b {
        println!("Yes");
    } else {
        println!("No");
    }
}
