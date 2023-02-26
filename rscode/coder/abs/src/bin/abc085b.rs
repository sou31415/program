use proconio::{fastout, input};
use std::collections::HashSet;
#[fastout]
fn main() {
    input! {
        n:usize , d:[usize;n],
    }
    let mut set = HashSet::new();
    for i in d {
        set.insert(i);
    }
    println!("{}", set.len());
}
