fn fizz_buzz(num: u32) -> Vec<String> {
    (1..=num)
        .map(|n| {
            let num_str = n.to_string();
            let mut result = String::new();

            if n % 3 == 0 {
                result.push_str("Fizz");
            }

            if n % 5 == 0 {
                result.push_str("Buzz");
            }

            if result.is_empty() {
                result.push_str(&num_str);
            }

            if num_str.contains('3') {
                result.push_str("Lucky");
            }

            result
        })
        .collect()
}

fn main() {
    let results = fizz_buzz(35);

    for (i, v) in results.iter().enumerate() {
        println!("{} => {}", i + 1, v);
    }
}
