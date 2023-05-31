use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        n:usize,abc:[(usize,usize,usize);n]
    }
    let mut dp: Vec<Vec<usize>> = vec![vec![0; 3]; n];
    dp[0][0] = abc[0].0;
    dp[0][1] = abc[0].1;
    dp[0][2] = abc[0].2;
    for i in 1..n {
        dp[i][0] = max(dp[i - 1][1] + abc[i].0, dp[i - 1][2] + abc[i].0);
        dp[i][1] = max(dp[i - 1][0] + abc[i].1, dp[i - 1][2] + abc[i].1);
        dp[i][2] = max(dp[i - 1][1] + abc[i].2, dp[i - 1][0] + abc[i].2);
    }
    println!("{}", max(dp[n - 1][0], max(dp[n - 1][1], dp[n - 1][2])));
}
