use petgraph::unionfind::UnionFind;
use proconio::input;
fn main() {
    input! {
        n:usize , q:usize,
    }
    let mut uf = UnionFind::new(n);
    for _ in 0..q {
        input! {
            t:usize , u:usize , v:usize,
        }
        if t == 0 {
            uf.union(u, v);
        } else {
            if uf.equiv(u, v) {
                println!("1");
            } else {
                println!("0");
            }
        }
    }
}
