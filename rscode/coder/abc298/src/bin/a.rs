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
        a:Chars,
    }
    if !a.contains(&'x') && a.contains(&'o') {
        println!("Yes");
    } else {
        println!("No");
    }
}
