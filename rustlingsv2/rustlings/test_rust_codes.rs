fn main() {
    let word = "A man a plan a canal Panama";
    let reversed_word: String = word.chars().rev().collect();
    let binding = word.to_lowercase();
    let cleaned_word = binding.replace(" ", "");
    let reversed_cleaned_word: String = cleaned_word.chars().rev().collect();

    fn is_palindrome(word: &str) -> bool {
        word.chars().eq(word.chars().rev())
    }

    println!("---------------------------------------------------------------");
    println!("|  Palindrome Check...");
    println!("| ---------------------------------------------------------");
    println!("|               ");
    println!("| The string is: {}", word);
    println!("| The reversed string is: {}", reversed_word);
    println!(
        "| is it a palindrome Before Triming: {}",
        is_palindrome(word)
    );
    println!("| ---------------------------------------------------------");
    println!("| Cleaned string: {}", cleaned_word);
    println!("| Reversed Cleaned string: {}", reversed_cleaned_word);
    println!(
        "| is it a palindrome After Triming: {}",
        is_palindrome(&cleaned_word)
    );
    println!("| ---------------------------------------------------------");
}
