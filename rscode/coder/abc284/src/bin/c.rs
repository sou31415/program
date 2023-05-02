use petgraph::unionfind::UnionFind;
use proconio::{fastout, input, marker::Usize1};
#[fastout]
fn main() {
    input! {
        n:usize , m:usize,
        ab:[(Usize1 , Usize1);m],
    }
    let mut uf = UnionFind::new(n);
    for &(a, b) in &ab {
        uf.union(a, b);
    }
    let mut set: Vec<usize> = uf.into_labeling();
    set.sort();
    set.dedup();
    println!("{}", set.len());
}
