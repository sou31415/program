#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars, marker::Usize1};
#[allow(unused_imports)]
use std::collections::{HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        s:String , t:String,
    }

    println!(
        "{}",
        if s.matches(&t).count() == 0 {
            "No"
        } else {
            "Yes"
        }
    );
}
