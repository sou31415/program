use proconio::fastout;
use proconio::input;
use std::collections::VecDeque;
#[fastout]
fn main() {
    input! {
        n:usize,a:[usize;n] , b:[usize;n],
    }
    let mut que = VecDeque::new();
    let mut result = 0;
    for i in 0..n {
        que.push_back(a[i]);
    }
    let mut de: usize = 0;
    for j in 0..16000 {
        for i in 0..n {
            de += que[i] | b[i];
        }
        if j == 0 {
            result = de;
        } else {
            if result < de {
                result = de;
            }
        }
        de = 0;
        let decoy = que.pop_front().unwrap();
        que.push_back(decoy);
    }
    println!("{}", result);
}
