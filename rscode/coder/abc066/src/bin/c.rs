use proconio::{fastout, input};
use std::collections::VecDeque;
#[fastout]
fn main() {
    input! {
        n:usize , a:[usize;n],
    }
    let mut que: VecDeque<usize> = VecDeque::new();
    for i in 0..n {
        if i % 2 == 0 {
            que.push_back(a[i]);
        } else {
            que.push_front(a[i]);
        }
    }
    if n % 2 == 0 {
        for i in que.iter() {
            print!("{} ", i);
        }
    } else {
        for i in 0..n {
            print!("{} ", que[n - 1 - i]);
        }
    }
}
