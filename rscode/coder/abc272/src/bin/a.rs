use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n:usize ,
        a:[usize;n],
    }
    println!("{}", a.into_iter().sum::<usize>());
}
