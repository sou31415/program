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
        s:[Chars;h],
    }
    for i in 0..h {
        for j in 0..w - 4 {
            if s[i][j] == 's'
                && s[i][j + 1] == 'n'
                && s[i][j + 2] == 'u'
                && s[i][j + 3] == 'k'
                && s[i][j + 4] == 'e'
            {
                for k in j..=(j + 4) {
                    println!("{} {}", i + 1, k + 1);
                }
                return;
            } else if s[i][j] == 'e'
                && s[i][j + 1] == 'k'
                && s[i][j + 2] == 'u'
                && s[i][j + 3] == 'n'
                && s[i][j + 4] == 's'
            {
                for k in (j..=(j + 4)).rev() {
                    println!("{} {}", i + 1, k + 1);
                }
                return;
            }
        }
    }
    for i in 0..h - 4 {
        for j in 0..w {
            if s[i][j] == 's'
                && s[i + 1][j] == 'n'
                && s[i + 2][j] == 'u'
                && s[i + 3][j] == 'k'
                && s[i + 4][j] == 'e'
            {
                for k in i..=(i + 4) {
                    println!("{} {}", k + 1, j + 1);
                }
                return;
            }
            if s[i][j] == 'e'
                && s[i + 1][j] == 'k'
                && s[i + 2][j] == 'u'
                && s[i + 3][j] == 'n'
                && s[i + 4][j] == 's'
            {
                for k in (i..=(i + 4)).rev() {
                    println!("{} {}", k + 1, j + 1);
                }
                return;
            }
        }
    }
    for i in 0..(h - 4) {
        for j in 0..(w - 4) {
            if s[i][j] == 's'
                && s[i + 1][j + 1] == 'n'
                && s[i + 2][j + 2] == 'u'
                && s[i + 3][j + 3] == 'k'
                && s[i + 4][j + 4] == 'e'
            {
                for k in 1..=5 {
                    println!("{} {}", i + k, j + k);
                }
                return;
            } else if s[i][j] == 'e'
                && s[i + 1][j + 1] == 'k'
                && s[i + 2][j + 2] == 'u'
                && s[i + 3][j + 3] == 'n'
                && s[i + 4][j + 4] == 's'
            {
                for k in (1..6).rev() {
                    println!("{} {}", i + k, j + k);
                }
                return;
            }
        }
    }
    for i in 4..h {
        for j in 0..(w - 4) {
            if s[i][j] == 's'
                && s[i - 1][j + 1] == 'n'
                && s[i - 2][j + 2] == 'u'
                && s[i - 3][j + 3] == 'k'
                && s[i - 4][j + 4] == 'e'
            {
                for k in 0..5 {
                    println!("{} {}", i + 1 - k, j + 1 + k);
                }
                return;
            } else if s[i][j] == 'e'
                && s[i - 1][j + 1] == 'k'
                && s[i - 2][j + 2] == 'u'
                && s[i - 3][j + 3] == 'n'
                && s[i - 4][j + 4] == 's'
            {
                for k in (0..5).rev() {
                    println!("{} {}", i + 1 - k, j + 1 + k);
                }
            }
        }
    }
}
