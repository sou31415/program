#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars, marker::Usize1};
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{BTreeSet, HashSet, VecDeque};
#[fastout]
fn main() {
    input! {
        q:usize,
    }
    let mut modu: usize = 1;
    let mut que: VecDeque<usize> = VecDeque::new();
    que.push_back(1);
    for _ in 0..q {
        input! {
            a:usize,
        }
        if a == 1 {
            input! {
                b:usize,
            }
            que.push_back(b);
            modu = (((modu * 10) % 998244353) + b) % 998244353;
        } else if a == 2 {
            let l = que.len();
            let d = que.pop_front().unwrap();
            modu = max(modu, (10_usize.pow(d as u32) as usize * l) % 998244353)
                - min(
                    modu,
                    ((10_usize.pow(d as u32) as usize * l) % 998244353) % 998244353,
                );
        } else {
            println!("{}", modu);
        }
    }
}
