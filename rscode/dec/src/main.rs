use proconio::input;
fn main() {
    input! {
        s:String,
    }
    for i in s.split('.').map(|x| x.parse::<usize>().unwrap()) {
        print!("{:0>8b}.", i);
    }
    println!("");
}
