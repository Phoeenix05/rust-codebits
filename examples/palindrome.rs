fn palindrome_str(string: String) -> bool {
    string.chars().rev().eq(string.chars())
    // true
}

fn palindrome_int(int: i32) -> bool {
    int.to_string().chars().rev().eq(int.to_string().chars())
    // true
}

/// Returns true if the input string or number reads the same from both ends. 
/// -121 is not a palidrome as it is 121- when read backwards.
fn main() {
    // let str_result = palindrome_str(String::from("Hello"));
    dbg!(palindrome_str(String::from("Hello")) == false);

    // let int_result = palindrome_int(121);
    dbg!(palindrome_int(121) == true);
}
