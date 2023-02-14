use proconio::{input, marker::Chars};
fn main() {
    input! {
        a:Chars,
    }
    let mut b = 0;
    for i in 0..a.len() {
        if a[i] == 'a' {
            b = i + 1;
        }
    }
    println!("{}", if (b == 0) { -1 } else { b as isize });
}
