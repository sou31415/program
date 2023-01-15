//submit passed
use proconio::{input , marker::Chars};
fn main() {
    input! {
        s:Chars,T:Chars,
    }
    let mut judge = 0;
    if s.len() <= T.len(){
        for i in 0..s.len() {
            if s[i] != T[i] {
                judge += 1;
                break;
            }
        }
        if judge == 0 {
            println!("Yes");
        }else {
            println!("No");
        }
    }else{
        println!("No");
    }
}
