#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars, marker::Usize1};
#[allow(unused_imports)]
use std::collections::{HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        a:[usize;6],
    }
    let mut result: usize = 0;
    let mut result2: usize = 0;
    let MOD: usize = 998244353;
    for i in 0..3 {
        result *= a[i] % MOD;
        result %= MOD;
    }
    for j in 3..6 {
        result2 *= a[j] % MOD;
        result2 %= MOD;
    }
    if result > result2 {
        println!("{}", result - result2);
    } else {
        println!("{}", MOD - (result2 - result));
    }
}
