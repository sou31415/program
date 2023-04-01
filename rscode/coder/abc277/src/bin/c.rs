use petgraph::unionfind::UnionFind;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n:usize,
        ab:[(Usize1 , Usize1);n],
    }
    let mut g: Vec<usize> = vec![]; //座標圧縮用
    g.push(0);
    for &(a, b) in &ab {
        g.push(a);
        g.push(b);
    }
    g.sort();
    g.dedup();
    let mut uf = UnionFind::new(g.len() + 3);
    for (a, b) in ab {
        let l: usize = g.binary_search(&a).unwrap();
        let r: usize = g.binary_search(&b).unwrap();
        uf.union(l, r);
    }
    let mut decoy: usize = 0;
    for i in 0..g.len() {
        if uf.equiv(0, i) {
            decoy = i;
        }
    }
    println!("{}", g[decoy] + 1);
}
