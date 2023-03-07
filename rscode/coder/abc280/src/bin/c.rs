#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars, marker::Usize1};
#[allow(unused_imports)]
use std::collections::{HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        s:Chars , t:Chars,
    }
    let mut result = t.len();
    for i in 0..s.len() {
        if s[i] != t[i] {
            result = i + 1;
            break;
        }
    }
    println!("{}", result);
}
