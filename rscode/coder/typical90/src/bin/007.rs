use proconio::{fastout, input};
use std::cmp::{max, min};
#[fastout]
fn main() {
    input! {
        n:usize,mut a:[usize;n],
        q:usize,b:[usize;q],
    }
    a.sort();
    for i in 0..q {
        if let Ok(x) = a.binary_search(&b[i]) {
            println!("0");
        } else if let Err(x) = a.binary_search(&b[i]) {
            if b[i] > a[n - 1] {
                println!("{}", b[i] - a[n - 1]);
            } else if b[i] < a[0] {
                println!("{}", a[0] - b[i]);
            } else {
                println!(
                    "{}",
                    min(
                        max(a[x], b[i]) - min(a[x], b[i]),
                        max(a[x - 1], b[i]) - min(a[x - 1], b[i])
                    )
                );
            }
        }
    }
}
