#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars, marker::Usize1};
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{BTreeSet, HashSet, VecDeque};
const MOD: usize = 998244353;
#[fastout]
fn main() {
    input! {
        q:usize,
    }
    let mut result: usize = 1;
    let mut que: VecDeque<usize> = VecDeque::new();
    que.push_back(1);
    let mut dp: Vec<usize> = vec![1];
    for i in 1..=600000 {
        let len = dp.len();
        let m = (dp[len - 1] * 10) % MOD;
        dp.push(m);
    }
    for _ in 0..q {
        input! {
            a:usize,
        }
        if a == 1 {
            input! {
                x:usize
            }
            que.push_back(x);
            result = (result * 10 + x) % MOD;
        } else if a == 2 {
            let len = que.len();
            let r = que.pop_front().unwrap();
            result = (result + 998244353 - (r * dp[len - 1]) % MOD) % MOD;
        } else {
            println!("{}", result);
        }
    }
}
