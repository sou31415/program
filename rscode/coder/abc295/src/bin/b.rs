#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars, marker::Usize1};
#[allow(unused_imports)]
use std::collections::{BTreeSet, HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        r:usize , c:usize,
        b:[Chars;r],
    }
    let mut result = vec![vec![','; 70]; 70];
    for i in 0..r {
        for j in 0..c {
            result[20 + i][20 + j] = b[i][j];
        }
    }
    for i in 0..r {
        for j in 0..c {
            if b[i][j] != '#' && b[i][j] != '.' {
                let d: usize = ((b[i][j] as i32) - 48_i32) as usize;
                for k in 0..=d {
                    for l in 0..=d - k {
                        result[(20 + i + k) as usize][20 + j + l] = '.';
                        result[(20 + i - k) as usize][20 + j - l] = '.';
                        result[(20 + i + k) as usize][20 + j - l] = '.';
                        result[(20 + i - k) as usize][20 + j + l] = '.';
                    }
                }
            }
        }
    }
    for i in 20..20 + r {
        for j in 20..20 + c {
            print!("{}", result[i][j]);
        }
        println!("");
    }
}
