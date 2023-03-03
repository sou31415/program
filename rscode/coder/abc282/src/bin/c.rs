use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n:usize , s:Chars,
    }
    let mut state = 0;
    for i in 0..n {
        if s[i] == '"' {
            state += 1;
        }
        if state % 2 == 0 && s[i] == ',' {
            print!(".");
        } else {
            print!("{}", s[i]);
        }
    }
}
