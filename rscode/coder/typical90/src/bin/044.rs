use proconio::{fastout, input};
use std::collections::VecDeque;
#[fastout]
fn main() {
    input! {
        n:usize,q:usize,
        a:[usize;n],
        ab:[(usize,usize,usize);q],
    }
    let mut que: VecDeque<usize> = a.iter().map(|&x| x).collect();
    for (a, x, y) in ab {
        if a == 1 {
            let k = que[x - 1];
            let f = que[y - 1];
            que[x - 1] = f;
            que[y - 1] = k;
        } else if a == 2 {
            let k = que.pop_back().unwrap();
            que.push_front(k);
        } else {
            println!("{}", que[x - 1]);
        }
    }
}
