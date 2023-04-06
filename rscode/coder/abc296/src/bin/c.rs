#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars, marker::Usize1};
#[allow(unused_imports)]
use std::collections::{BTreeSet, HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        n:usize , x:i128,
        mut a:[i128;n],
    }
    if x == 0 {
        println!("Yes");
        return;
    }
    a.sort();
    for i in 0..n {
        if let Ok(b) = a.binary_search(&(a[i] as i128)) {
            if let Ok(_) = a.binary_search(&(a[i] as i128 - x as i128)) {
                println!("Yes");
                return;
            } else if let Ok(_) = a.binary_search(&(x as i128 + a[i])) {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
