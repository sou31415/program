#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars, marker::Usize1};
#[allow(unused_imports)]
use std::collections::{BTreeSet, HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        n:usize , m:usize,
    }
    if (n * n) < m {
        println!("-1");
        return;
    }
    let mut ans: usize = std::usize::MAX;
    for i in 1..=n {
        let x = (m + i - 1) / i;
        if x <= n {
            ans = std::cmp::min(ans, i * x);
        }
        if i > x {
            break;
        }
    }
    println!("{}", ans);
}
