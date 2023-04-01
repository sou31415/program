#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars, marker::Usize1};
#[allow(unused_imports)]
use std::collections::{BTreeSet, HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        s:[Chars;8],
    }
    let mut x: char = 'A';
    let mut y: usize = 0;
    for i in 0..8 {
        for j in 0..8 {
            if s[i][j] == '*' {
                y = 8 - i - 1;
                x = (j as u8 + 'a' as u8) as char;
            }
        }
    }
    println!("{}{}", x as char, y + 1);
}
