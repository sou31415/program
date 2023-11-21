use proconio::input;

fn main() {
    input! {
        n:usize,mut s:[String;n]
    }
    s.reverse();
    for i in 0..n {
        println!("{}", s[i]);
    }
}
