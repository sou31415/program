use proconio::{input, marker::Chars};

fn main() {
    input! {
        a:usize , b:usize,
        c:[Chars;a],
    }
    let mut result: Vec<usize> = vec![0; b];
    for i in 0..b {
        for j in 0..a {
            if c[j][i] == '#' {
                result[i] += 1;
            }
        }
        print!("{} ", result[i]);
    }
}
