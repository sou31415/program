use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};

fn main() {
    input! {
      n:usize,m:usize,ab:[(Usize1,Usize1);m],
    }
    let mut cnt: usize = 0;
    let mut uf = UnionFind::new(n);
    for &(a, b) in ab.iter() {
        if !uf.union(a, b) {
            cnt += 1;
        }
    }
    println!("{}", cnt);
}
