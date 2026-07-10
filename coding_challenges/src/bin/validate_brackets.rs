fn validate_bracket(s: &str) -> bool {
    let has_bracket = s
        .chars()
        .any(|c| matches!(c, '(' | ')' | '{' | '}' | '[' | ']'));

    if has_bracket == false {
        return false;
    } else {
        true
    }
}

fn main() {
    let s = "hello (world)";

    let has_bracket = validate_bracket(s);

    println!("{}", has_bracket); // true
}
