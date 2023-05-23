use itertools::Itertools;
use proconio::{fastout, input, marker::Usize1};
#[fastout]
fn main() {
    input! {
        n:usize,m:usize,d:usize,k:usize,
        a:[(Usize1,Usize1,usize);m],
    }
    println!("{}", (1..=m).map(|x| (x % d) + 1).join(" "));
}
