#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars, marker::Usize1};
#[allow(unused_imports)]
use std::collections::{HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        s:Chars,
    }
    for i in 0..s.len() / 2 {
        print!("{}", s[i * 2 + 1]);
        print!("{}", s[i * 2]);
    }
}
