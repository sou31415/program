use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};
use std::collections::HashSet;
#[proconio::fastout]
fn main() {
    input! {
        n :usize , m:usize,
        ab:[(Usize1 , Usize1);m],
    }
    let mut uf = UnionFind::new(n);
    for (a, b) in ab {
        uf.union(a, b);
    }
    let set = uf.into_labeling().into_iter().collect::<HashSet<usize>>();
    println!("{}", set.len());
}
