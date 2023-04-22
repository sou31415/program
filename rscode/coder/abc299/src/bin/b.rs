#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars, marker::Usize1};
#[allow(unused_imports)]
use std::collections::{BTreeSet, HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        n:usize , t:usize,
        c:[usize;n],
        r:[usize;n],
    }
    let set: HashSet<usize> = c.iter().map(|x| *x).collect();
    if set.contains(&t) {
        let mut idx: usize = 0;
        let mut pa: usize = 0;
        for i in 0..n {
            if c[i] == t && idx <= r[i] {
                idx = r[i];
                pa = i;
            }
        }
        println!("{}", pa + 1);
    } else {
        let mut pa: usize = 0;
        let mut idx: usize = 0;
        for i in 0..n {
            if idx < r[i] && c[i] == c[0] {
                pa = i;
                idx = r[i];
            }
        }
        println!("{}", pa + 1);
    }
}
