#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars};
#[fastout]
fn main() {
    input! {
        a:usize , b:usize,
        mut s:Chars,
    }
    let mut pass = 0;
    for i in 0..s.len() {
        if pass == b && s[i] == 'o' {
            s[i] = 'x';
        } else if s[i] == 'o' {
            pass += 1;
        }
    }
    for i in 0..s.len() {
        print!("{}", s[i]);
    }
}
