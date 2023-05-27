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
        n:usize,s:Chars,t:Chars,
    }
    for i in 0..n {
        if s[i] == 'o' && t[i] == '0'
            || t[i] == 'o' && s[i] == '0'
            || s[i] == 'l' && t[i] == '1'
            || (s[i] == '1' && t[i] == 'l')
            || s[i] == t[i]
        {
            continue;
        } else {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
