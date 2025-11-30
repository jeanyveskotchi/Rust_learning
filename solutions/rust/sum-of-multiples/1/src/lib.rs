use std::collections::HashSet;
pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
let mut set: HashSet<u32> = HashSet::new();
    let mut i:u32=1;
    for &x in factors{
    if x==0{
        continue;
    }
        let mut m=x;
while m<limit{
    set.insert(m);
    m+=x;
}      
    }
       set.iter().sum()
}
