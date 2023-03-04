#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars, marker::Usize1};
#[allow(unused_imports)]
use std::collections::{HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        n:usize , q:usize,
        ab:[(usize , Usize1);q],
    }

    let mut data: Vec<usize> = vec![0_usize; n];
    for (a, b) in ab {
        if a == 1 {
            data[b] += 1;
        } else if a == 2 {
            data[b] += 2;
        } else {
            if data[b] >= 2 {
                println!("Yes");
            } else {
                println!("No");
            }
        }
    }
}
