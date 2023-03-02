use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n:usize , s:Chars ,
    }
    for i in 1..n {
        let mut result: usize = 0;
        let mut state: bool = false;
        for j in 0..n - i {
            if s[j] != s[i + j] {
                result += 1;
            } else {
                println!("{}", result);
                state = true;
                break;
            }
        }
        if !state {
            println!("{}", result);
        }
    }
}
