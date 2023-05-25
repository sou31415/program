use proconio::{input, marker::Chars};
use std::cmp::{max, min};
fn main() {
    input! {
        n:usize,s:Chars,
    }
    let mut xcnt: usize = 0;
    let mut ocnt: usize = 0;
    let mut sigma: usize = n * (n + 1) / 2;
    let mut v: Vec<usize> = vec![];
    for i in 0..n {
        if s[i] == 'x' {
            if ocnt != 0 {
                v.push(ocnt);
                ocnt = 0;
            }
            xcnt += 1;
        } else {
            if xcnt != 0 {
                v.push(xcnt);
                xcnt = 0;
            }
            ocnt += 1;
        }
    }
    v.push(max(xcnt, ocnt));
    let k: usize = v.iter().map(|&x| x * (x + 1) / 2).sum();
    println!("{}", sigma - k);
}
