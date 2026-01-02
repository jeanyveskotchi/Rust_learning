pub fn series(digits: &str, len: usize) -> Vec<String> {
let mut v:Vec<String>=Vec::new();
    
    if len==0 || len>digits.len(){
        return v
    }
    for count in 0..=digits.len() - len {
        let sub=&digits[count..count+len];
        v.push(sub.to_string());
    }
v
}
