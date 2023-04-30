#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars, marker::Usize1};
#[allow(unused_imports)]
use std::collections::{BTreeSet, HashSet, VecDeque};

#[fastout]
fn main() {
    input! {
        h:usize,w:usize,
        mut a:[String;h],b:[Chars;h],
    }
    let mut ac: VecDeque<VecDeque<char>> = VecDeque::new();
    while let Some(x) = a.pop() {
        let y = x.chars().collect::<VecDeque<char>>();
        ac.push_front(y);
    }
    for _ in 0..h {
        if judge(h, w, ac.clone(), b.clone()) {
            println!("Yes");
            return;
        }
        for _ in 0..w {
            if judge(h, w, ac.clone(), b.clone()) {
                println!("Yes");
                return;
            }
            for k in 0..h {
                let y = ac[k].pop_back().unwrap();
                ac[k].push_front(y);
            }
            if judge(h, w, ac.clone(), b.clone()) {
                println!("Yes");
                return;
            }
        }
        if judge(h, w, ac.clone(), b.clone()) {
            println!("Yes");
            return;
        }
        let m = ac.pop_back().unwrap();
        ac.push_front(m);
        if judge(h, w, ac.clone(), b.clone()) {
            println!("Yes");
            return;
        }
    }
    if judge(h, w, ac.clone(), b.clone()) {
        println!("Yes");
        return;
    }
    println!("No");
}

fn judge(h: usize, w: usize, a: VecDeque<VecDeque<char>>, b: Vec<Vec<char>>) -> bool {
    for i in 0..h {
        for k in 0..w {
            if a[i][k] != b[i][k] {
                return false;
            }
        }
    }
    true
}
