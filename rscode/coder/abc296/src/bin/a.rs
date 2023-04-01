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
        s:Chars,
    }
    for i in 0..n - 1 {
        if s[i] == s[i + 1] {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
