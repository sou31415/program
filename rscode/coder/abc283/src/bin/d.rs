//submittion not passed
use proconio::{input , marker::Chars};
use std::collections::HashSet;
fn main() {
    input!{
        a:Chars,
    }
    let set = HashSet::new();
    for i in 0..a.len() {
        if a[i] >=  "a" && a[i] <= "z" {
            if a.contains(&a[i]) == false {
                break;
                println!("No");
            }else{
                set.insert(&a[i]);
            }
        }else if a[i] == ")" {

}
