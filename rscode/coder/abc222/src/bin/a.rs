use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:String,
    }
    for _ in 0..4 - s.len() {
        print!("0");
    }
    print!("{}", s);
}
