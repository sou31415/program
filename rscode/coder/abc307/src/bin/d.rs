#![allow(unused_imports)]
use itertools::Itertools;
use petgraph::unionfind::UnionFind;
use proconio::{fastout, input, marker::Chars, marker::Usize1, source::line::LineSource};
use std::cmp::{max, min};
use std::collections::{BTreeSet, HashSet, VecDeque};
use std::io::{stdin, stdout, BufReader};
#[fastout]
fn main() {
    input! {
        n:usize,st:String,
    }
    let s = st.chars().collect::<Vec<char>>();
    let mut set: VecDeque<usize> = VecDeque::new();
    let mut set2: VecDeque<usize> = VecDeque::new();
    for i in 0..s.len() {
        if s[i] == '(' {
            set.push_back(i);
        } else if s[i] == ')' {
            set2.push_back(i);
        }
    }
    let mut f: Vec<bool> = vec![false; n];
    if st.matches("(").count() == 0 || st.matches(")").count() == 0 {
        println!("{}", st);
        return;
    }

    while set.len() != set2.len() {
        if set.len() < set2.len() {
            set2.pop_back();
        } else {
            set.pop_front();
        }
    }
    let len2 = set2.len();
    for i in 0..s.len() {
        if i < set[0] || i > set2[len2 - 1] {
            print!("{}", s[i]);
        }
    }
}
