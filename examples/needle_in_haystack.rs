/// Return the first index (occurance) of needle in haystack.
/// If needle isn't in the haystack then this function returns -1.
fn solution(haystack: String, needle: String) -> i32 {
    haystack
        .find(needle.as_str())
        .map(|i| i as i32)
        .unwrap_or(-1)
}

fn main() {
    let result = solution("happybuthappy".to_string(), "happy".to_string());
    dbg!(result == 0);
}
