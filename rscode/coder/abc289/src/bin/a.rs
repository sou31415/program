use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        n:Chars,
    }
    for i in 0..n.len() {
        if n[i] == '0' {
            print!("1");
        } else {
            print!("0");
        }
    }
}
