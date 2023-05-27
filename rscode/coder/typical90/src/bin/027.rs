use proconio::{fastout, input};
use std::collections::HashSet;
#[fastout]
fn main() {
    input! {
        n:usize,a:[String;n],
    }
    let mut set: HashSet<&String> = HashSet::new();
    for i in 0..n {
        if !set.contains(&&a[i]) {
            set.insert(&a[i]);
            println!("{}", i + 1);
        }
    }
}
