use proconio::{input , fastout}
#[fastout]
fn main(){
  input!{
    step:usize,
  }
  let mut result :usize = 0;
  if step == 1{
    input!{
        n:usize , x : usize , y : usize , 
    }
    for i in 0..n{
        input!{
            ab:[(usize , usize);n],
        }
    }
    for (a , b) in ab{
        result += b - a;
    }
    println!("{}" , result);
  }
  else if step == 2{
      let mut dp:Vec<usize> = vec![];
      input!{
         n:usize , x:usize , y:usize , ab:[(usize , usize);n],
      }
      for (a , b) in ab{
          dp.push(b-a);
      }
  }
}
