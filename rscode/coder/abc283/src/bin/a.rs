//submit passed
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a:usize , b:usize,
    }
    let result = power(a, b);
    println!("{}", result);
}

fn power(a: usize, b: usize) -> usize {
    a.pow(b as u32)
}
