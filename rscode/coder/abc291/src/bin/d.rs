use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize , ab:[[usize;2];n],
    }
    let mut dp: Vec<Vec<usize>> = vec![vec![0; 2]; n];
    dp[0] = vec![1, 1];
    for i in 1..n {
        for j in 0..2_usize {
            for k in 0..2_usize {
                if ab[i - 1][j] != ab[i][k] {
                    dp[i][k] += dp[i - 1][j];
                }
            }
        }
        dp[i][0] %= 998244353_usize;
        dp[i][1] %= 998244353_usize;
    }
    println!("{}", (dp[n - 1][0] + dp[n - 1][1]) % 998244353_usize);
}
