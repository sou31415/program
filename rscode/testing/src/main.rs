#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars, marker::Usize1};
#[allow(unused_imports)]
use std::collections::{HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        n:usize , q:usize,
    }
    let mut waitb: Vec<usize> = vec![];
    let mut wait: VecDeque<usize> = (0..n).collect();
    for _ in 0..q {
        input! {
            cas:usize,
        }
        if cas == 1 {
            let b: usize = wait.pop_front().unwrap();
            waitb.push(b);
        } else if cas == 2 {
            input! {
                x:Usize1,
            }
            let idx: usize = waitb.binary_search(&x).unwrap();
            waitb.swap_remove(idx);
        } else {
            println!("{}", waitb[0] + 1);
        }
    }
}
