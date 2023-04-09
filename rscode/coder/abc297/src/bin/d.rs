#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars, marker::Usize1};
#[allow(unused_imports)]
use std::collections::{BTreeSet, HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        mut a:usize , mut b:usize,
    }
    let mut cnt: usize = 0;
    while a != b {
        if a > b {
            if a % b != 0 {
                let d: usize = a / b;
                a -= d * b;
                cnt += d;
            } else {
                let d: usize = (a / b) - 1;
                a -= d * b;
                cnt += d;
            }
        } else {
            if b % a != 0 {
                let d: usize = b / a;
                b -= d * a;
                cnt += d;
            } else {
                let d: usize = (b / a) - 1;
                b -= d * a;
                cnt += d;
            }
        }
    }
    println!("{}", cnt);
}
