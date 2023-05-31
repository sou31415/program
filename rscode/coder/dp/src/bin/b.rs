use proconio::input;
use std::cmp::{max, min};

fn main() {
    input! {
        n:usize,k:usize,
        a:[usize;n],
    }
    let mut dp: Vec<usize> = vec![0_usize; n];
    for i in 1..n {
        let mut cost: usize = std::usize::MAX;
        for j in 1..=min(i, k) {
            if cost > (dp[i - j] + max(a[i], a[i - j]) - min(a[i], a[i - j])) {
                cost = (dp[i - j] + max(a[i], a[i - j]) - min(a[i], a[i - j]));
            }
        }
        dp[i] = cost;
    }
    println!("{}", dp[n - 1]);
}
