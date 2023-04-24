use petgraph::unionfind::UnionFind;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [(String, String); n]
    }
    let mut v: Vec<String> = Vec::new();
    let mut uf: UnionFind<usize> = UnionFind::new(2 * n);
    for (a, b) in &s {
        v.push(a.to_string());
        v.push(b.to_string());
    }
    v.sort();
    v.dedup();
    for (a, b) in &s {
        if uf.equiv(v.binary_search(&a).unwrap(), v.binary_search(&b).unwrap()) {
            println!("No");
            return;
        } else {
            uf.union(v.binary_search(&a).unwrap(), v.binary_search(&b).unwrap());
        }
    }
    println!("Yes");
}
