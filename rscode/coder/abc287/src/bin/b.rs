use proconio::input;
fn main() {
    input! {
        a:usize,b:usize,
        c:[String;a],
        mut d:[String;b],
    }
    let mut cnt: usize = 0;
    let mut set = std::collections::HashSet::new();
    for i in d {
        set.insert(i);
    }
    for i in c.iter() {
        for j in set.iter() {
            if i[3..=5] == j.to_string() {
                cnt += 1;
            }
        }
    }
    println!("{}", cnt);
}
