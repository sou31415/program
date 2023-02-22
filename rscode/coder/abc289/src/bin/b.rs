#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars, marker::Usize1};
#[fastout]
fn main() {
    input! {
        n:usize , m:usize,
        a:[Usize1;m],
    }
    let mut stats: Vec<bool> = vec![false; n];
    for i in 0..m {
        stats[a[i]] = true;
    }
    let mut j: usize = 0;
    let mut result: Vec<usize> = vec![];
    while j < n {
        if stats[j] {
            result.push(j);
            //result.push(j + 1);
            j += 1;
        } else if !stats[j] && result.len() > 0 {
            print!("{} ", j + 1);
            result.reverse();
            j += 1;
            for i in 0..result.len() {
                print!("{} ", result[i] + 1);
            }
            result = vec![];
        } else {
            print!("{} ", j + 1);
            j += 1;
        }
    }
}
