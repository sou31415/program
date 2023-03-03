use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        mut s:Chars,
    }
    s.reverse();
    let mut result: usize = 0;
    for i in 0..s.len() {
        result += ((s[i] as u8) - ('A' as u8) + 1_u8) as usize * power(26, &i);
    }
    println!("{}", result);
}
fn power(a: usize, b: &usize) -> usize {
    a.pow(*b as u32)
}
