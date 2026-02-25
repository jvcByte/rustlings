fn main() {
    let l1 = vec![2, 2, 2];
    let l2 = vec![3, 3, 3];

    let mut return_num: i64 = 0;

    for i in 0..l1.len() {
        return_num += l1[i] as i64 * 10_i64.pow(i as u32)
    }

    for i in 0..l2.len() {
        return_num += l2[i] as i64 * 10_i64.pow(i as u32)
    }

    println!("{}", return_num);
}
