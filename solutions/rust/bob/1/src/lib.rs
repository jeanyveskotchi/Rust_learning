pub fn reply(message: &str) -> &str {
 let msg=message.trim();
let has_letter=msg.chars().any(|c| c.is_alphabetic());
let is_yelling=has_letter && msg==msg.to_uppercase();
 
   if is_yelling && msg.ends_with('?'){
      "Calm down, I know what I'm doing!"
  }   
  else if msg.ends_with('?') {
      "Sure."
  }
   else if is_yelling{
      "Whoa, chill out!"
  }
   else if msg.is_empty(){
      "Fine. Be that way!"
  }
    else{
    "Whatever."
    }
}
