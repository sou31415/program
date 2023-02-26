#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use proconio::{fastout, input, marker::Chars, marker::Usize1};
#[allow(unused_imports)]
use std::collections::HashSet;
#[fastout]
fn main() {
    input! {
        n:usize , a:Chars,
    }
    let mut jd: bool = false;
    let mut set = HashSet::new();
    let mut place: (isize, isize) = (0, 0);
    set.insert(place);
    for i in 0..n {
        if a[i] == 'L' {
            place.0 = place.0 - 1;
        } else if a[i] == 'R' {
            place.0 = place.0 + 1;
        } else if a[i] == 'U' {
            place.1 = place.1 + 1;
        } else if a[i] == 'D' {
            place.1 = place.1 - 1;
        } else {
            continue;
        }
        if set.contains(&place) {
            jd = true;
            println!("Yes");
            break;
        } else {
            set.insert(place);
        }
    }
    if !jd {
        println!("No");
    }
}
