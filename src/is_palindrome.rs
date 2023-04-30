pub fn is_palindrome_setup(){
    let x = 101101101;
    let r = is_palindrome(x);
    println!("{r}");
}

pub fn is_palindrome(x: i32) -> bool {
    let str = x.to_string();
    let str_len = str.len();
    let mut ch:Vec<char> = Vec::new();

    for character in str.chars() {
        ch.push(character);
    }

    for index in 0..str_len {
        let j = str_len - index - 1;
        if index > j {
            break;
        }
        if ch[index] != ch[j]{
            return false;
        }
    }
    
    true
}