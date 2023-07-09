use proconio::{input, marker::Chars};
use std::cmp::min;
fn main() {
    input! {
      x:usize,y:usize,z:usize,
      s:Chars,
    }
    let n = s.len();
    let mut dp: Vec<Vec<usize>> = vec![vec![0; 2]; n];
    if s[0] == 'a' {
        dp[0][0] = x;
        dp[0][1] = z + y;
    } else {
        dp[0][0] = y;
        dp[0][1] = z + x;
    }
    for i in 1..n {
        if s[i] == 'a' {
            dp[i][0] = min(dp[i - 1][0] + x, dp[i - 1][1] + z + x);
            dp[i][1] = min(dp[i - 1][1] + y, dp[i - 1][0] + z + y);
        } else {
            dp[i][1] = min(dp[i - 1][0] + z + x, dp[i - 1][1] + x);
            dp[i][0] = min(dp[i - 1][0] + y, dp[i - 1][1] + z + y);
        }
    }
    println!("{}", min(dp[n - 1][0], dp[n - 1][1]));
}
