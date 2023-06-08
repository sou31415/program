#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars, marker::Usize1};
#[allow(unused_imports)]
use std::collections::{BTreeSet, HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        n:usize,m:usize,d:usize,
    }
    input! {
        mut a:[usize;n],
        mut b:[usize;m],
    }
    a.sort();
    b.sort();
    while a.len() > 0 && b.len() > 0 {
        let r = a.len() - 1;
        let l = b.len() - 1;
        if a[r] > b[l] {
            if a[r] - b[l] <= d {
                println!("{}", a[r] + b[l]);
                return;
            }
            a.pop();
        } else {
            if b[l] - a[r] <= d {
                println!("{}", b[l] + a[r]);
                return;
            }
            b.pop();
        }
    }
    println!("-1");
}
