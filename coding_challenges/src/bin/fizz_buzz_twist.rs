fn fizzbuzz_twist(start: u32, end: u32) -> Vec<String> {
    (start..=end)
        .map(|num| {
            let mut result = String::new();
            let num_str = num.to_string();

            if num % 3 == 0 {
                result.push_str("Fizz");
            }

            if num % 5 == 0 {
                result.push_str("Buzz");
            }

            if num % 7 == 0 {
                result.push_str("Bazz");
            }

            if result.is_empty() {
                result = num_str;
            }

            result
        })
        .collect()
}

fn main() {
    let result = fizzbuzz_twist(1, 35);

    for (i, v) in result.iter().enumerate() {
        println!("{} => {}", i + 1, v);
    }
}
