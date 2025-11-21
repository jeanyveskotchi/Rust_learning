pub fn reverse(input: &str) -> String {
    let mut result =String::new();
    for ch in input.chars().rev(){
        result.push(ch);
    }
    result
}
