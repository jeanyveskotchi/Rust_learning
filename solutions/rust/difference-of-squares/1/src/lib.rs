pub fn square_of_sum(n: u32) -> u32 {
    let mut result: u32=0;
    let mut i=0;
    while(i<=n){
        result+=i;
        i+=1;
    }
    result.pow(2)
}

pub fn sum_of_squares(n: u32) -> u32 {
    let mut result: u32=0;
    let mut i=0;
    while i<=n{
        result+=i.pow(2);
        i+=1;
    }
    result
}

pub fn difference(n: u32) -> u32 {
 square_of_sum(n) - sum_of_squares(n)  
}
