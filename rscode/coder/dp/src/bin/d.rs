use proconio::input;
use std::cmp::max;
fn main() {
    input! {
        n:usize,w:usize,
        vw:[(usize,usize);n],
    }
    // dp[i][j] := i番目までの荷物を詰めた中で重量をjにする時の価値の総和
    let mut dp: Vec<Vec<usize>> = vec![vec![0; w + 1]; n];
    let d: usize = vw[0].0;
    for i in d..=w {
        dp[0][i] = vw[0].1;
    }
    for i in 1..n {
        for j in 0..=w {
            if j + vw[i].0 <= w {
                dp[i][j + vw[i].0] = max(dp[i - 1][j + vw[i].0], dp[i - 1][j] + vw[i].1);
            } else {
                dp[i][w - j] = max(dp[i][w - j], dp[i - 1][w - j]);
            }
        }
    }
    println!("{}", dp[n - 1][w]);
}
