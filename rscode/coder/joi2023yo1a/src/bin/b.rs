use proconio::{input, marker::Chars};
fn main() {
    input! {
        a:Chars,
    }
    if a[0] == a[1] {
        println!("{}", 1);
    } else {
        println!("{}", 0);
    }
}
