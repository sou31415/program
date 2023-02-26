use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        mut n:usize , a:usize , b:usize,
    }
    let mut result: Vec<usize> = vec![];
    for i in 1..=n {
        let mut decoy = i;
        let ch = total(&mut decoy);
        if ch >= a && ch <= b {
            result.push(i);
        }
    }
    let sum: usize = result.iter().sum();
    println!("{}", sum);
}

fn total(c: &mut usize) -> usize {
    let mut sum = 0;
    while *c != 0 {
        sum += *c % 10;
        *c /= 10;
    }
    sum
}
