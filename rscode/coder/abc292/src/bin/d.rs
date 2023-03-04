#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars, marker::Usize1};
#[allow(unused_imports)]
use std::collections::{HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        n:usize , m:usize ,
        _ab:[(Usize1 , Usize1);m],
    }
    println!("{}", if n == m { "Yes" } else { "No" });
}
