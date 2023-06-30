#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars, marker::Usize1};
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{BTreeSet, HashSet, VecDeque};
const MOD: usize = 998244353;
#[fastout]
fn main() {
    input! {
        q:usize,
    }
    let mut que: VecDeque<usize> = VecDeque::new();
    que.push_back(1);
    let mut dp: Vec<Vec<usize>> = vec![10];
}
