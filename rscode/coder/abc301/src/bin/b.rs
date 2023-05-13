#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars, marker::Usize1};
#[allow(unused_imports)]
use std::collections::{BTreeSet, HashSet, VecDeque};

fn main() {
    input! {
        n:usize,mut a:[usize;n]
    }
    let mut jd = false;
    for i in 0..(n - 1) {
        print!("{} ", a[i]);
        if a[i] + 1 == a[i + 1] || a[i] - 1 == a[i + 1] {
            continue;
        } else if a[i] > a[i + 1] {
            for j in (a[i + 1] + 1..a[i]).rev() {
                print!("{} ", j);
            }
        } else if a[i] < 1 + a[i + 1] {
            for j in (a[i] + 1..a[i + 1]) {
                print!("{} ", j);
            }
        }
    }
    println!("{}", a[n - 1]);
}
