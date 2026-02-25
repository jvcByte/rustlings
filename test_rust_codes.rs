fn add_two_nums(l1: Vec<u32>, l2: Vec<u32>) -> Vec<u32> {
    let mut hold_num: i64 = 0;

    for i in 0..l1.len() {
        hold_num += l1[i] as i64 * 10_i64.pow(i as u32);
        println!("L1 Index {} is {}", i, l1[i]);
    }

    for i in 0..l2.len() {
        hold_num += l2[i] as i64 * 10_i64.pow(i as u32);
        println!("L2 Index {} is {}", i, l2[i]);
    }

    let return_vec: Vec<u32> = hold_num
        .to_string()
        .chars()
        .rev()
        .map(|c| c.to_digit(10).unwrap())
        .collect();

    return_vec
}

fn main() {
    let l1 = vec![9, 9, 9, 9, 9, 9, 9];
    let l2 = vec![9, 9, 9, 9];
    let result = add_two_nums(l1, l2);
    println!(" Main Result: {result:?}");
}
