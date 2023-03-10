use proconio::{fastout, input, marker::Usize1};

#[fastout]
fn main() {
    input! {
        n:usize , m:usize ,
        ab:[(Usize1 , Usize1);m],
    }
    let mut data: Vec<Vec<usize>> = vec![vec![]; n];
    for &(a, b) in &ab {
        data[a].push(b);
        data[b].push(a);
    }
    for i in 0..n {
        data[i].sort();
        print!("{} ", data[i].len());
        for j in 0..data[i].len() {
            print!("{} ", data[i][j] + 1);
        }
        println!("");
    }
}
