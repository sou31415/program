use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize,
    }
    let mut result: usize = 1;
    for i in 1..=n {
        result *= i;
    }
    println!("{}", result);
}
