use std::collections::HashMap;

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut seen = HashMap::new();
    for (i, &num) in nums.iter().enumerate() {
        if let Some(&j) = seen.get(&(target - num)) {
            return vec![j, i as i32];
        }
        seen.insert(num, i as i32);
    }
    vec![]
}

fn main() {
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let result = two_sum(nums, target);
    println!("{result:?}");
}
