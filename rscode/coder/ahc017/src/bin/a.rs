use petgraph::unionfind::UnionFind;
use proconio::{fastout, input, marker::Usize1};
#[fastout]
fn main() {
    input! {
        n:usize,m:usize,d:usize,k:usize,
        a:[(Usize1,Usize1,usize);m],
    }
    let mut uf = UnionFind::new(n);
    for (a,b,_) in a{
        uf.union(a,b);
    }
    :q
}
