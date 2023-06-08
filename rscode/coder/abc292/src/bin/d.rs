#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars, marker::Usize1};
#[allow(unused_imports)]
use std::collections::{HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        n:usize , m:usize ,
        ab:[(Usize1 , Usize1);m],
    }
    if n != m {
        println!("No");
        return;
    }
    let mut g: Vec<Vec<usize>> = vec![vec![]; n];
    let mut uf = UnionFind::new(n);
    for (a, b) in ab {
        uf.union(a, b);
        g[a].push(b);
        g[b].push(a);
    }
    let mut q = uf.clone().into_labeling();
    let s = q.clone();
    q.sort();
    q.dedup();
    let mut v: Vec<usize> = vec![0; q.len()];
    for i in 0..n {
        let a = q.binary_search(&(uf.find(i))).unwrap();
        v[a] += g[i].len();
    }
    for i in 0..q.len() {
        if (v[i] / 2) != s.iter().filter(|&&x| x == q[i]).count() {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
