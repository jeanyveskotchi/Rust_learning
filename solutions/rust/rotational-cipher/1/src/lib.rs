pub fn rotate(input: &str, key: u8) -> String {
   let mut cipher=String::new();
    for ch in input.chars(){
    if ch.is_ascii_lowercase(){
        let rotated=
            ((ch as u8 - b'a' +key)%26 + b'a') as char;
        cipher.push(rotated);
    }   
      else if ch.is_ascii_uppercase(){
        let rotated=
            ((ch as u8 - b'A' +key)%26 + b'A') as char;
        cipher.push(rotated);
    }
        else{
           cipher.push(ch);
        }
    }
    cipher
}
