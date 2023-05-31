use proconio::input;
use std::cmp::{max, min};

fn main() {
    input! {
        n:usize,
        h:[usize;n],
    }
    let mut dp: Vec<usize> = vec![0; n];
    dp[1] = max(h[0], h[1]) - min(h[0], h[1]);
    for i in 2..n {
        let cost = min(
            dp[i - 1] + max(h[i - 1], h[i]) - min(h[i - 1], h[i]),
            dp[i - 2] + max(h[i - 2], h[i]) - min(h[i - 2], h[i]),
        );
        dp[i] = cost;
    }
    println!("{}", dp[n - 1]);
}
