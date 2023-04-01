#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars, marker::Usize1};
#[allow(unused_imports)]
use std::collections::{BTreeSet, HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        n:u128 , m:u128,
    }
    if (n * n) < m {
        println!("-1");
        return;
    }
    for i in m..=(n * n) {
        for j in 1..=((m as f64).sqrt() as usize) {
            if i % j as u128 == 0 && j as u128 <= n && (i as u128 / j as u128) <= n {
                println!("{}", i as u128);
                return;
            }
        }
    }
}
