#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s:String,
    }
    let mut length = s.len() - 2;
    loop {
        if s[0..(length / 2 - 1)] == s[length / 2..length - 1] {
            break;
        }
        length -= 2;
    }
    println!("{}", length);
}
