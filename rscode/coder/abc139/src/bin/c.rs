use proconio::input;
use std::cmp::{max, min};
fn main() {
    input! {
      n:usize,a:[usize;n],
    }
    let mut cnt2: usize = 0;
    let mut cnt: usize = 0;
    for i in 0..n - 1 {
        if a[i] >= a[i + 1] {
            cnt += 1;
            cnt2 = max(cnt, cnt2);
        } else {
            cnt2 = max(cnt, cnt2);
            cnt = 0;
        }
    }
    println!("{}", cnt2);
}
