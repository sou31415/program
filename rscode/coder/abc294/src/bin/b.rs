#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars, marker::Usize1};
#[allow(unused_imports)]
use std::collections::{HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        h:usize , w:usize,
        a:[[usize;w];h],
    }
    for i in 0..h {
        for j in 0..w {
            if a[i][j] == 0 {
                print!(".");
            } else {
                print!("{}", ('A' as u8 - 1_u8 + a[i][j] as u8) as char);
            }
        }
        println!("");
    }
}
