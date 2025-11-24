/// Determines whether the supplied string is a valid ISBN number
pub fn is_valid_isbn(isbn: &str) -> bool {
    let mut int:u32=10;
    let mut result=0;
    let mut seen=0; //count the number of digit seen
    
    for ch in isbn.chars(){
        if ch=='-'{
            continue; //skip the '-'
        }

     if int==0 {
     return false; // isbn too long
        }
        
        let digit =if ch=='X' && int==1{
            10 // X is allowed as last digit 
        }
            else if let Some(d)=ch.to_digit(10){
                d  //an integer (0-9)
            }
            else{
                return false;
            };
        result+=digit*int;
        int-=1;
        seen+=1;
    }
    result%11==0 && seen==10
}
