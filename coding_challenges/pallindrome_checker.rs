fn is_palindrome(s: &str) -> bool {
    let cleaned_string: String = s
        .trim()
        .to_lowercase()
        .chars()
        .filter(|c| c.is_alphabetic())
        .collect();

    cleaned_string.chars().eq(cleaned_string.chars().rev())
}

fn main() {
    let result = is_palindrome("Racecar");
    println!("{}", result);
}
