#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars, marker::Usize1};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{BTreeSet, HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        n:usize,m:usize,h:usize,k:usize,
        s:Chars,
        xy:[(i64,i64);m]
    }
    let mut set: BTreeSet<(i64, i64)> = xy.iter().map(|&(a, b)| (a, b)).collect();
    let mut stam: i64 = h as i64;
    let mut place: (i64, i64) = (0, 0);

    for i in 0..n {
        if s[i] == 'R' {
            place.0 += 1;
        } else if s[i] == 'L' {
            place.0 -= 1;
        } else if s[i] == 'U' {
            place.1 += 1;
        } else {
            place.1 -= 1;
        }
        stam -= 1;
        if stam < 0 {
            println!("No");
            return;
        }
        if stam < k as i64 && set.contains(&place) {
            stam = k as i64;
            set.remove(&place);
        }
    }
    println!("Yes");
}
