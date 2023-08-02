use proconio::{input, marker::Chars};

fn main() {
    input! {
        h:usize,w:usize,
        s:[Chars;h],
    }
    let mut f = false;
    let mut d = false;
    let mut dp: Vec<Vec<usize>> = vec![vec![0; w]; h];
    for i in 0..w {
        if s[0][i] == '#' {
            f = true;
        }
        if s[0][i] == '.' && !f {
            dp[0][i] = 1;
        }
    }
    let m: usize = 1000000007;
    for i in 0..h {
        if s[i][0] == '#' {
            d = true;
        }
        if s[i][0] == '.' && !d {
            dp[i][0] = 1;
        }
    }
    for i in 1..h {
        for j in 1..w {
            if s[i][j] == '.' {
                dp[i][j] = (dp[i - 1][j] + dp[i][j - 1]) % m;
            }
        }
    }
    println!("{}", dp[h - 1][w - 1]);
}
