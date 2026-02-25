

fn add_two_nums(l1: Vec<i32>, l2: Vec<i32>) ->


fn main() {
    let l1 = vec![2, 4, 3];
    let l2 = vec![5, 6, 4];

    let mut return_num: i64 = 0;

    for i in 0..l1.len() {
        return_num += l1[i] as i64 * 10_i64.pow(i as u32)
    }

    for i in 0..l2.len() {
        return_num += l2[i] as i64 * 10_i64.pow(i as u32)
    }

    println!("{}", return_num);
}
