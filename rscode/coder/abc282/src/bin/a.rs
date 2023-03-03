use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        s:usize,
    }
    let str: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    for i in 0..s {
        print!("{}", str[i]);
    }
}
