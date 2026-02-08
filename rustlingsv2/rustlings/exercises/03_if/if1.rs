fn bigger(a: i32, b: i32) -> i32 {
    // TODO: Complete this function to return the bigger number!
    // If both numbers are equal, any of them can be returned.
    // Do not use:
    // - another function call
    // - additional variables
    if a >= b { a } else { b }
}

fn main() {
    // You can optionally experiment here.
    let val1 = bigger(25, 30);
    println!("Bigger number is: {val1}");

    let val2 = bigger(45, 25);
    println!("Bigger number is: {val2}");

    let val3 = bigger(15, 15);
    println!("Bigger number is: {val3}");
}

// Don't mind this for now :)
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }

    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }

    #[test]
    fn equal_numbers() {
        assert_eq!(42, bigger(42, 42));
    }
}
