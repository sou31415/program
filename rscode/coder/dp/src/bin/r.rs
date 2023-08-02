use proconio::input;
const MOD: usize = 1000000007;
fn main() {
    input! {
      n:usize,k:usize,
      s:[[usize;n];n]
    }
    let mut cnt: usize = 0;
    let r = matrix_pow(s, MOD, k);
    for i in 0..n {
        cnt += r[i].iter().sum::<usize>();
        cnt %= MOD;
    }
    println!("{}", cnt);
}
pub fn matrix_pow(mut r: Vec<Vec<usize>>, m: usize, mut x: usize) -> Vec<Vec<usize>> {
    let mut v: Vec<Vec<usize>> = vec![vec![0; r.len()]; r.len()];
    for i in 0..r.len() {
        v[i][i] = 1;
    }
    let mut i: usize = 0;
    while x != 0 {
        if 1usize << i & x != 0 {
            let mut d: Vec<Vec<usize>> = vec![vec![0; r.len()]; r.len()];
            for i in 0..r.len() {
                for j in 0..r.len() {
                    for k in 0..r.len() {
                        d[i][j] += v[i][k] * r[k][j];
                        d[i][j] %= m;
                    }
                }
            }
            x -= 1usize << i;
            v = d;
        }
        let mut d: Vec<Vec<usize>> = vec![vec![0; r.len()]; r.len()];
        for i in 0..r.len() {
            for k in 0..r.len() {
                for j in 0..r.len() {
                    d[i][j] += r[i][k] * r[k][j];
                    d[i][j] %= m;
                }
            }
        }
        r = d;
        i += 1;
    }
    return v;
}
