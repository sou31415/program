use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        a:usize , b:usize , k:usize,
    }
    let mut v: Vec<usize> = vec![];
    for i in 1..=100 {
        if a % i == 0 && b % i == 0 {
            v.push(i);
        }
    }
    v.reverse();
    println!("{}", v[k - 1]);
}
