pub fn is_armstrong_number(num: u32) -> bool {
    let a=num.to_string();
    let power=a.len() as u32;
let mut result: u32=0;
    
    for ch in a.chars(){
    let mut digit= ch.to_digit(10).unwrap();
    result+=digit.pow(power);   
    }
    result ==num
}
