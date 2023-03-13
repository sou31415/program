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
    let Mod: usize = 998244353;
    let result1 = (((a[0] % Mod) * (a[1] % Mod) % Mod) * (a[2] % Mod)) % Mod;
    let result2 = (((a[3] % Mod) * (a[4] % Mod) % Mod) * (a[5] % Mod)) % Mod;
    if result1 >= result2 {
        println!("{}", result1 - result2);
    } else {
        println!("{}", 998244353 - (result2 - result1));
    }
}
