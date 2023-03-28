use proconio::{input, marker::Usize1};
fn main() {
    input! {
        n :usize , m:usize,
        ab:[(Usize1 , Usize1);m],
    }
    let mut uf = petgraph::unionfind::UnionFind::new(n);
    for (a, b) in ab {
        uf.union(a, b);
    }
    println!(
        "{}",
        uf.into_labeling()
            .into_iter()
            .collect::<std::collections::HashSet<_>>()
            .len()
    );
}
