use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};
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
    let mut set = std::collections::HashSet::new();
    for i in 0..n {
        set.insert(uf.find(i));
    }
    println!("{}", set.len());
}
