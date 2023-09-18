#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars, marker::Usize1};
#[allow(unused_imports)]
use std::collections::{HashSet, VecDeque};

fn main() {
    input! {
        a:usize,j:usize,mut d:usize,
    }
    let r: Vec<Vec<usize>> = vec![vec![a, 1], vec![0, 1]];
    let k = matrix_pow(r, d, j);
    let result = k[0][1];
    println!("{}", result.fact);
}
pub fn matrix_pow(mut s: Vec<Vec<usize>>, m: usize, mut x: usize) -> Vec<Vec<Modint>> {
    let mut r: Vec<Vec<Modint>> = vec![vec![Modint::new(m, 0); s.len()]; s.len()];
    for i in 0..s.len() {
        for j in 0..s.len() {
            r[i][j] = Modint::new(m, s[i][j]);
        }
    }
    let mut v: Vec<Vec<Modint>> = vec![vec![Modint::new(m, 0usize); r.len()]; r.len()];
    for i in 0..r.len() {
        v[i][i] = Modint::new(m, 1usize);
    }
    let mut i: usize = 0;
    while x != 0 {
        if 1usize << i & x != 0 {
            let mut d: Vec<Vec<Modint>> = vec![vec![Modint::new(m, 0usize); r.len()]; r.len()];
            for i in 0..r.len() {
                for j in 0..r.len() {
                    for k in 0..r.len() {
                        d[i][j] += v[i][k] * r[k][j];
                    }
                }
            }
            x ^= 1usize << i;
            v = d;
        }
        let mut d: Vec<Vec<Modint>> = vec![vec![Modint::new(m, 0usize); r.len()]; r.len()];
        for i in 0..r.len() {
            for k in 0..r.len() {
                for j in 0..r.len() {
                    d[i][j] += r[i][k] * r[k][j];
                }
            }
        }
        r = d;
        i += 1;
    }
    return v;
}

use std::ops;

#[derive(Debug, Clone, Eq, PartialEq, Copy)]
#[allow(non_snake_case)]
pub struct Modint {
    pub MOD: usize,
    pub fact: usize,
}

impl Modint {
    pub fn new(first_mod: usize, init: usize) -> Modint {
        Modint {
            MOD: first_mod,
            fact: init,
        }
    }
}

impl ops::Add for Modint {
    type Output = Modint;
    fn add(self, other: Self) -> Self {
        Modint::new(self.MOD, (self.fact + other.fact) % self.MOD)
    }
}
impl ops::Mul for Modint {
    type Output = Modint;
    fn mul(self, other: Self) -> Self {
        Modint::new(self.MOD, (self.fact * other.fact) % self.MOD)
    }
}
impl ops::Div for Modint {
    type Output = Modint;
    fn div(self, other: Self) -> Self {
        Modint::new(self.MOD, self.fact / other.fact)
    }
}
impl ops::Sub for Modint {
    type Output = Modint;
    fn sub(self, other: Self) -> Self {
        Modint::new(self.MOD, (self.MOD + self.fact + other.fact) % self.MOD)
    }
}
impl ops::AddAssign for Modint {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}
impl ops::SubAssign for Modint {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}
impl ops::MulAssign for Modint {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}
