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
        n:usize,s:String
    }
    if s.matches("T").count() > s.matches("A").count() {
        println!("T");
    } else if s.matches("T").count() < s.matches("A").count() {
        println!("A");
    } else {
        if s.chars().nth(n - 1).unwrap() == 'A' {
            println!("T");
        } else {
            println!("A");
        }
    }
}
