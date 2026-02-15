fn main() {
    let word = "A man a plan a canal Panama";
    let reversed_word: String = word.chars().rev().collect();

    fn is_palindrome(word: &str) -> bool {
        word.chars().eq(word.chars().rev())
    }

    println!("The word is: {}", word);
    println!("The reversed word is: {}", reversed_word);
    println!("is it a palindrome: {}", is_palindrome(word));
}
