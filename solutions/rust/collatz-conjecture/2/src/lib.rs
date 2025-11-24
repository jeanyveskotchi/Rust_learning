pub fn collatz(n: u64) -> Option<u64> {
   if n==0{
       return None;
   }
    let mut count:u64= 0;
    let mut current=n;
    while current!=1{
if current%2==0{
    current/=2;
}
else{
    current=current*3+1; 
}
        count+=1;
    }
    Some(count)   
}
