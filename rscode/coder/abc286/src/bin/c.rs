use std::collections::VecDeque;

use proconio::{input, marker::Chars};
fn main() {
    input! {
        n:usize,a:usize,b:usize,
        l:String
    }
    let mut result: usize = 1 << 63;
    let mut s: VecDeque<char> = l.chars().collect::<VecDeque<char>>();
    for i in 1..=n {
        let r = s.pop_front().unwrap();
        s.push_back(r);
        let q = rotate_diff(&s);
        result = std::cmp::min((i % n) * a + q * b, result);
    }
    println!("{}", result);
}
pub fn rotate(s: String) -> bool {
    let q = s.chars().collect::<Vec<char>>();
    let n = q.len();
    for i in 0..(n / 2) {
        if q[i] != q[n - i - 1] {
            return false;
        }
    }
    return true;
}

pub fn rotate_diff(s: &VecDeque<char>) -> usize {
    let n = s.len();
    let mut result: usize = 0;
    for i in 0..(n / 2) {
        if s[i] != s[n - i - 1] {
            result += 1;
        }
    }
    result
}
