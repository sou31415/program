//submit passed
use proconio::input;
use std::collections::HashSet;
fn main() {
    input! {
        a:[usize;5],
    }
    //let mut cnt = 5;
    let mut s = HashSet::new();
    for i in a {
        s.insert(i);
    }
    println!("{}", s.len());
}
