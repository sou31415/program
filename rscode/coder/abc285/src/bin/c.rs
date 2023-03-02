use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        mut s:Chars,
    }
    s.reverse();
    let mut result: usize = 0;
    for i in 0..s.len() {
        match s[i] {
            'A' => result += power(26, &i),
            'B' => result += 2 * power(26, &i),
            'C' => result += 3 * power(26, &i),
            'D' => result += 4 * power(26, &i),
            'E' => result += 5 * power(26, &i),
            'F' => result += 6 * power(26, &i),
            'G' => result += 7 * power(26, &i),
            'H' => result += 8 * power(26, &i),
            'I' => result += 9 * power(26, &i),
            'J' => result += 10 * power(26, &i),
            'K' => result += 11 * power(26, &i),
            'L' => result += 12 * power(26, &i),
            'M' => result += 13 * power(26, &i),
            'N' => result += 14 * power(26, &i),
            'O' => result += 15 * power(26, &i),
            'P' => result += 16 * power(26, &i),
            'Q' => result += 17 * power(26, &i),
            'R' => result += 18 * power(26, &i),
            'S' => result += 19 * power(26, &i),
            'T' => result += 20 * power(26, &i),
            'U' => result += 21 * power(26, &i),
            'V' => result += 22 * power(26, &i),
            'W' => result += 23 * power(26, &i),
            'X' => result += 24 * power(26, &i),
            'Y' => result += 25 * power(26, &i),
            'Z' => result += 26 * power(26, &i),
            _ => result += 0,
        }
    }
    println!("{}", result);
}
fn power(a: usize, b: &usize) -> usize {
    a.pow(*b as u32)
}
