use proconio::{fastout, input, marker::Chars};

#[fastout]
fn main() {
    input! {
        n:usize,a:[Chars;n],
    }
    let mut set = std::collections::HashSet::new();
    for i in 0..n {
        set.insert(a[i].clone());
    }
    let mut jd: bool = false;
    if a.len() != set.len() {
        println!("No");
        jd = true;
    } else {
        for i in 0..n {
            if a[i][0] == 'H' || a[i][0] == 'D' || a[i][0] == 'C' || a[i][0] == 'S' {
                if a[i][1] == 'A'
                    || a[i][1] == '2'
                    || a[i][1] == '3'
                    || a[i][1] == '4'
                    || a[i][1] == '5'
                    || a[i][1] == '6'
                    || a[i][1] == '7'
                    || a[i][1] == '8'
                    || a[i][1] == '9'
                    || a[i][1] == 'T'
                    || a[i][1] == 'Q'
                    || a[i][1] == 'J'
                    || a[i][1] == 'K'
                {
                    continue;
                } else {
                    println!("No");
                    jd = true;
                    break;
                }
            } else {
                println!("No");
                jd = true;
                break;
            }
        }
    }
    if !jd {
        println!("Yes");
    }
}
