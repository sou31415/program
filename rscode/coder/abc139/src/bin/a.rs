use proconio::{input, marker::Chars};
fn main() {
    input! {
        s:Chars,c:Chars,
    }
    println!("{}", (0..3).filter(|&x| s[x] == c[x]).count());
}
