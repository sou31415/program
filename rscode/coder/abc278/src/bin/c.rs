#[allow(unused_imports)]
use proconio::{fastout, input, marker::Usize1};
use std::collections::HashSet;

#[fastout]
fn main() {
    input! {
        _n:usize,q:usize,
        ab:[(usize , usize , usize);q],
    }
    let mut set = HashSet::new();
    for (a, b, c) in ab {
        if a == 1 {
            set.insert((b, c));
        } else if a == 2 {
            set.remove(&(b, c));
        } else {
            if set.contains(&(b, c)) && set.contains(&(c, b)) {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}
