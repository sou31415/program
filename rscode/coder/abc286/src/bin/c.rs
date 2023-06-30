use proconio::{input, marker::Chars};
use std::collections::VecDeque;
fn main() {
    input! {
      n:usize,a:usize,b:usize,
      x:Chars,
    }
    let mut s = x.iter().map(|&x| x).collect::<VecDeque<char>>();
    let mut result: usize = b * n / 2;
    result = std::cmp::min(result, rotate(n, &s) * b);
    for i in 1..=n {
        let k = s.pop_front().unwrap();
        s.push_back(k);
        let m = rotate(n, &s);
        result = std::cmp::min(result, i * a + m * b);
    }
    println!("{}", result);
}

fn rotate(n: usize, s: &VecDeque<char>) -> usize {
    let mut result: usize = 0;
    for i in 0..(n / 2) {
        if s[i] != s[n - i - 1] {
            result += 1;
        }
    }
    result
}
