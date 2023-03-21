use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        s:Chars,
    }
    let mut z: usize = 0;
    let mut o: usize = 0;
    for i in 0..s.len() {
        if s[i] == '0' {
            o += 1;
        } else {
            z += 1;
        }
    }
    println!("{}", std::cmp::min(o, z) * 2);
}
