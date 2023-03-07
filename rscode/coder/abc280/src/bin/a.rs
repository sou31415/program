#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars, marker::Usize1};
#[allow(unused_imports)]
use std::collections::{HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        h:usize , _w:usize ,
        s:[Chars;h],
    }
    let mut count: usize = 0;
    for i in 0..h {
        count += s[i].iter().filter(|x| x == &&'#').count();
    }
    println!("{}", count);
}
