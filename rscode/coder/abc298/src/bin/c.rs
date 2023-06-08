use itertools::Itertools;
#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars, marker::Usize1};
use std::collections::BinaryHeap;
#[allow(unused_imports)]
use std::collections::{BTreeSet, HashSet, VecDeque};
#[fastout]
fn main() {
    input! {
        n:usize,
        q:usize,
    }
    let mut s = vec![BTreeSet::new();n];
    let mut set:BTreeSet<usize> = BTreeSet::new();
    for _ in 0..q{
        input!{
            a:usize,i:usize,
        }
        if a == 1{
            input!{
                j:Usize1,
            }

}
