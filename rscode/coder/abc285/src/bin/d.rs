use petgraph::unionfind::UnionFind;
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        s: [(String, String); n]
    }
    let mut v: Vec<String> = Vec::new();
    for (a, b) in s.iter() {
        v.push(a.to_string());
        v.push(b.to_string());
    }
    v.sort();
    v.dedup();
    let mut uf = UnionFind::new(v.len());
    for (a, b) in s.iter() {
        let s = v.binary_search(&a).unwrap();
        let r = v.binary_search(&b).unwrap();
        if !uf.union(s, r) {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
