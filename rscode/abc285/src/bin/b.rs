use proconio::{input,fastout};
#[fastout]
fn main() {
    input! {
        n:usize,
        s:String,
    }
    //let mut l:usize = 0;
    
    for i in 1..n {
        let mut l:usize = 0;
        let mut judge : isize = 0;
        for j in 0..(n-i){
            if s.chars().nth(0).unwrap() == s.chars().nth(i).unwrap() {
                judge = -1;
                break;
            }
            if s.chars().nth(j).unwrap() != s.chars().nth(j+i).unwrap() {
                if (j+i) <= n{
                    l = j;
                }
            }else{
                break;
            }
        }
        println!("{}" , judge+(1+l as isize));
        
    }
        //println!("{}" , l);
}

