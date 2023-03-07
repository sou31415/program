//submit passed
#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars, marker::Usize1};
#[allow(unused_imports)]
use std::collections::{HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        n:usize , mut t:usize ,
        a:[usize;n],
    }
    let total: usize = a.iter().sum();
    let mut count: usize = 0;
    t = t % total;
    for i in 0..n {
        if a[i] < t {
            count += 1;
            t -= a[i];
        } else {
            break;
        }
    }
    println!("{} {}", count + 1, t);
}
