use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};
fn main() {
    input! {
        n:usize,m:usize,ab:[(Usize1 , Usize1);m],
    }
    let mut result = UnionFind::new(n);
    let mut ans: usize = 0;
    for (a, b) in ab {
        if result.equiv(a, b) {
            ans += 1;
        } else {
            result.union(a, b);
        }
    }
    println!("{}", ans);
}
