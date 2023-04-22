#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars, marker::Usize1};
#[allow(unused_imports)]
use std::collections::{BTreeSet, HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        n:usize,
        a:Chars,
    }
    let mut b: (usize, usize) = (0, 0);
    let mut c: usize = 0;
    let mut bc: usize = 0;
    for i in 0..n {
        if a[i] == '|' && bc == 0 {
            b.0 = i;
            bc += 1;
        } else if a[i] == '|' && bc == 1 {
            b.1 = i;
            bc += 1;
        } else if a[i] == '*' {
            c = i;
        }
    }
    if b.0 < c && b.1 > c {
        println!("in");
    } else {
        println!("out");
    }
}
