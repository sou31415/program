#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars, marker::Usize1};
use std::cmp::{max, min};
#[allow(unused_imports)]
use std::collections::{BTreeSet, HashSet, VecDeque};
#[fastout]
fn main() {
    input! {
        mut s:Chars,mut c:Chars,
    }
    let mut v: Vec<usize> = vec![0; 7];
    let mut v1: Vec<usize> = vec![0; 7];
    let set = vec!['a', 't', 'c', 'o', 'd', 'e', 'r']
        .iter()
        .map(|&x| x)
        .collect::<HashSet<char>>();
    let mut a: Vec<usize> = vec![0; 27];
    let mut b: Vec<usize> = vec![0; 27];
    let mut cnt: (usize, usize) = (0, 0);
    let mut result: usize = 0;
    for i in 0..s.len() {
        if !set.contains(&s[i]) && s[i] != '@' {
            a[(s[i] as u8 - 'a' as u8 + 1) as usize] += 1;
        } else {
            match s[i] {
                'a' => v[0] += 1,
                't' => v[1] += 1,
                'c' => v[2] += 1,
                'o' => v[3] += 1,
                'd' => v[4] += 1,
                'e' => v[5] += 1,
                'r' => v[6] += 1,
                _ => cnt.0 += 1,
            }
        }
        if !set.contains(&c[i]) && c[i] != '@' {
            b[(c[i] as u8 - 'a' as u8 + 1) as usize] += 1;
        } else {
            match c[i] {
                'a' => v1[0] += 1,
                't' => v1[1] += 1,
                'c' => v1[2] += 1,
                'o' => v1[3] += 1,
                'd' => v1[4] += 1,
                'e' => v1[5] += 1,
                'r' => v1[6] += 1,
                _ => cnt.1 += 1,
            }
        }
    }
    if a != b {
        println!("No");
        return;
    }
    for i in 0..7 {
        result += max(v[i], v1[i]) - min(v[i], v1[i]);
    }
    if cnt.0 + cnt.1 >= result {
        println!("Yes");
    } else {
        println!("No");
    }
}
