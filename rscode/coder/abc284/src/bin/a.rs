//submit passed
#[allow(unused_imports)]
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n:usize,
        strlist:[String;n],
    }
    for i in 0..n {
        println!("{}", strlist[n - i - 1]);
    }
}
