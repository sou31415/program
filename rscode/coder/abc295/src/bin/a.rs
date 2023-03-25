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
        t:[String;n],
    }
    for i in t.iter() {
        if i == "and" || i == "not" || i == "the" || i == "that" || i == "you" {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
