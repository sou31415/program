use proconio::input;
fn main() {
    input! {
        a:usize,
        b:usize,
        mut c:[String;a],
    }
    let d = &mut c[..b];
    d.sort();
    for j in 0..d.len() {
        println!("{}", d[j]);
    }
}
