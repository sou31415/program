use proconio::{fastout, input};
use std::collections::HashSet;
#[fastout]
fn main() {
    input! {
        n:usize , k:usize,
        mut a:[usize;n],
    }
    let mut ans: usize = 0;
    a.sort();
    let mut result = HashSet::new();
    for i in 0..a.len() {
        result.insert(a[i]);
    }
    for i in 0..k {
        if result.contains(&i) {
            ans += 1;
        } else {
            break;
        }
    }
    println!("{}", ans);
}
