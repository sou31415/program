use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
    }
    println!("{}", s[(s.len() - 1) / 2]);
}
