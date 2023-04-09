#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars, marker::Usize1};
#[allow(unused_imports)]
use std::collections::{BTreeSet, HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        n:usize , w:usize,
        a:[usize;n],
    }
    for i in 0..n - 1 {
        if (a[i + 1] - a[i]) <= w {
            println!("{}", a[i + 1]);
            return;
        }
    }
    println!("-1");
}
