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
        x:usize,y:usize,z:usize,
        s:Chars,
    }
    let mut v = VecDeque::new();
    let mut l: usize = 0;
    let mut u: usize = 0;
    let mut c = false;
    let mut d = false;
    for i in 0..s.len() {
        if s[i] == 'A' {
            u += 1;
            if l != 0 {
                v.push_back(l);
                v.push_back(0);
                l = 0;
            }
        } else {
            l += 1;
            if u != 0 {
                v.push_back(u);
                v.push_back(0);
                u = 0;
            }
        }
    }
    if x == y {
        let q = v.iter().map(|&a| a * x).sum::<usize>();
        let r = v.iter().map(|&a| a * y).sum::<usize>();
        println!("{}", q + r);
        return;
    }
    if a[0] == 'a'{
        let q = z / max(x,y) - min(x,y);
        let n = v.pop_front().unwrap() * x;
        cnt += n;
        for i in 0..v.len(){
            if a[i] > 
}
