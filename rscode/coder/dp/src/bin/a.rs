use proconio::input;
use std::cmp::{max, min};

fn main() {
    input! {
        n:usize,
        h:[usize;n],
    }
    let mut dp: Vec<usize> = vec![0; n];
    dp[0] = h[0];
    let mut place: usize = 1;
    dp[1] = max(h[0], h[1]) - min(h[0], h[1]);
    place += 1;
    while place < n {
        let l = dp[place - 2] + (h[place - 2] as isize - h[place] as isize).abs() as usize;
        let r = dp[place - 1] + (h[place - 1] as isize - h[place] as isize).abs() as usize;
        if l < r {
            dp[place] = l;
        } else {
            dp[place] = r;
        }
        place += 1;
    }
    println!("{}", dp[n - 1]);
}
