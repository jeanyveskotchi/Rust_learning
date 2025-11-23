pub fn factors(mut n: u64) -> Vec<u64> {
    let mut m: u64=2;
    let mut ans:Vec<u64>= Vec::new();
    while n!=1{
if n%m==0{
    ans.push(m);
    n=n/m;
}
  else {
      m+=1;
  }    
    }
        ans
}
