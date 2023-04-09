#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars, marker::Usize1};
#[allow(unused_imports)]
use std::collections::{BTreeSet, HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        h:usize , _:usize,
        mut a:[String;h],
    }
    for i in a {
        println!("{}", i.replace("TT", "PC"));
    }
}
