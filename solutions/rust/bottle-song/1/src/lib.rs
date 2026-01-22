pub fn recite(start_bottles: u32, take_down: u32) -> String {
fn bottle(n: u32) -> &'static str {
    if n == 1 { "bottle" } else { "bottles" }
}
  fn number_to_word(n: u32) -> &'static str {
    match n {
        0 => "no",
        1 => "One",
        2 => "Two",
        3 => "Three",
        4 => "Four",
        5 => "Five",
        6 => "Six",
        7 => "Seven",
        8 => "Eight",
        9 => "Nine",
        10 => "Ten",
        _ => unreachable!(),
    }
}
    let mut lines:Vec<String>=Vec::new();
    for i in (start_bottles - take_down + 1..=start_bottles).rev() {
         let current = number_to_word(i);
    let next = number_to_word(i - 1);
       lines.push(format!(
    "{} green {} hanging on the wall,\n\
{} green {} hanging on the wall,\n\
And if one green bottle should accidentally fall,\n\
There'll be {} green {} hanging on the wall.",
    current,
    bottle(i),
    current,
    bottle(i),
    next.to_lowercase(),
    bottle(i-1),
));
    } 
lines.join("\n\n")
}
