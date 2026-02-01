use rand::Rng;

fn random_key(len: usize) -> String {
    let mut rng = rand::thread_rng();

    (0..len)
        .map(|_| (rng.gen_range(b'a'..=b'z') as char))
        .collect()
}

pub fn encode(key: &str, s: &str) -> Option<String> {
   if key.is_empty() || !key.chars().all(|c| c.is_ascii_lowercase()){
       return None;
   }
    let mut cipher=String::new();
    let mut iter2=key.chars().cycle();
    for c1 in s.chars(){
        if c1.is_ascii_lowercase(){
        let c2=iter2.next().unwrap().to_ascii_lowercase();
        let rotate=
            (c1 as u8 -b'a' + (c2 as u8 - b'a'))%26 + b'a';
        cipher.push(rotate as char);
    }
     else {
         cipher.push(c1);
     }
}
    Some(cipher)
   }
pub fn decode(key: &str, s: &str) -> Option<String> {
    if key.is_empty() || !key.chars().all(|c| c.is_ascii_lowercase()){
        return None;
    }
    
    let mut result = String::new();
    let mut iter2 = key.chars().cycle();

    for c1 in s.chars() {
        if c1.is_ascii_lowercase() {
            let c2 = iter2.next().unwrap().to_ascii_lowercase();
            let rot =
                (26 + (c1 as u8 - b'a') - (c2 as u8 - b'a')) % 26 + b'a';
            result.push(rot as char);
     } else {
            result.push(c1);
        }
    }

    Some(result)
}

pub fn encode_random(s: &str) -> (String, String) {
    let letters = s.chars().count();
   let key_len=letters.max(100);
    let key = random_key(key_len);
    let encoded = encode(&key, s).unwrap();
    (key,encoded)
}
