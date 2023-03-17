use petgraph::unionfind::UnionFind;
use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n:usize , m:usize,
        uv:[(Usize1 , Usize1);m],
    }
    let mut edge: Vec<Vec<usize>> = vec![vec![]; n];
    let mut uf = UnionFind::new(n);
    if n - 1 != m {
        println!("No");
        return;
    }
    for (a, b) in uv {
        if !uf.equiv(a, b) {
            uf.union(a, b);
            edge[a].push(b);
            edge[b].push(a);
        } else {
            println!("No");
            return;
        }
    }
    if edge.iter().filter(|x| x.len() == 2).count() == n - 2
        && edge.iter().filter(|x| x.len() == 1).count() == 2
    {
        println!("Yes");
    } else {
        println!("No");
    }
}
