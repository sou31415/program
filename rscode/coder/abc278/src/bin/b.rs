#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars, marker::Usize1};
#[allow(unused_imports)]
use std::collections::{HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        mut ab:usize , mut cd:usize,
    }
    solve(&mut ab, &mut cd);
    println!("{} {}", ab, cd);
}

fn solve(a: &mut usize, b: &mut usize) {
    let ac: usize = (*a / 10) * 10 + *b / 10;
    let bd: usize = (*a % 10) * 10 + *b % 10;
    if ac <= 23 && bd <= 59 {
        return;
    } else {
        if *b == 59 && *a == 23 {
            *a = 0;
            *b = 0;
            solve(a, b);
        } else if *b == 59 {
            *b = 0;
            *a += 1;
            solve(a, b);
        } else {
            *b += 1;
            solve(a, b);
        }
    }
}
