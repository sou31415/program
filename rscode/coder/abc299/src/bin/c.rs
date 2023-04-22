#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars, marker::Usize1};
#[allow(unused_imports)]
use std::collections::{BTreeSet, HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        mut n:usize,
        s:String,
    }
    let set = s.chars().collect::<HashSet<char>>();
    let v = s.split('-').map(|x| x.len()).max().unwrap();
    println!(
        "{}",
        if v == 0 || v == n || !set.contains(&'o') {
            -1
        } else {
            v as i64
        }
    );
}
