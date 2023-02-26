#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars, marker::Usize1};
#[allow(unused_imports)]
use std::collections::HashSet;
#[fastout]
fn main() {
    input! {
        a:Chars,
    }
    let mut ch: usize = 0;
    for i in 0..a.len() {
        if a[i].is_uppercase() {
            ch = i;
            break;
        }
    }
    println!("{}", ch + 1);
}
