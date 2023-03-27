use proconio::{input, marker::Usize1};

fn main() {
    input! {
        l:Usize1,r:Usize1,
    }
    let lis: Vec<char> = vec!['a', 't', 'c', 'o', 'd', 'e', 'r'];
    for i in l..=r {
        print!("{}", lis[i]);
    }
}
