use proconio::input;
use std::cmp::min;
fn main() {
    input! {
        n:usize,
        a:[usize;3],
    }
    let mut result: usize = std::usize::MAX;
    for i in 0..10000 {
        for j in 0..(10000 - i) {
            let k = a[0] * i + a[1] * j;
            if n >= k {
                if (n - k) % a[2] == 0 {
                    result = min(result, (n - k) / a[2] + i + j);
                }
            }
        }
    }
    println!("{}", result);
}
