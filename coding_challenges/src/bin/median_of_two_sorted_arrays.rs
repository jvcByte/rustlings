use rand::Rng;

struct Solution {
    nums1: Vec<i32>,
    nums2: Vec<i32>,
}
impl Solution {
    pub fn new(nums1: Vec<i32>, nums2: Vec<i32>) -> Self {
        Solution { nums1, nums2 }
    }

    pub fn find_median_sorted_arrays(&self) -> f64 {
        let total_len = self.nums1.len() + self.nums2.len();
        println!("Total length: {total_len}");

        if total_len % 2 == 0 {
            println!("Hit even case");
            return (self.find_median(&self.nums1, &self.nums2, total_len / 2)
                + self.find_median(&self.nums1, &self.nums2, total_len / 2 + 1))
                as f64
                / 2.0;
        } else {
            println!("Hit odd case");
            println!("K: {}", total_len / 2 + 1);
            return self.find_median(&self.nums1, &self.nums2, total_len / 2 + 1) as f64;
        }
    }

    fn find_median(&self, nums1: &[i32], nums2: &[i32], k: usize) -> i32 {
        if nums1.is_empty() {
            println!("Hit nums1 empty case");
            return nums2[k - 1];
        }
        if nums2.is_empty() {
            println!("Hit nums2 empty case");
            return nums1[k - 1];
        }

        if k == 1 {
            println!("Hit k == 1 case");
            return nums1[0].min(nums2[0]);
        }

        let mid1 = (k / 2).min(nums1.len());
        println!("mid1: {}", mid1);
        let mid2 = (k / 2).min(nums2.len());
        println!("mid2: {}", mid2);

        if nums1[mid1 - 1] <= nums2[mid2 - 1] {
            println!("Hit nums1[{}] <= nums2[{}] case", mid1 - 1, mid2 - 1);
            return self.find_median(&nums1[mid1..], &nums2, k - mid1);
        } else {
            println!("Hit nums1[{}] >= nums2[{}] case", mid1 - 1, mid2 - 1);
            return self.find_median(&nums1, &nums2[mid2..], k - mid2);
        }
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut big_vec: Vec<i32> = (0..1_000_000)
        .map(|_| rng.gen_range(-1_000_000..1_000_000))
        .collect();
    big_vec.sort();
    let solution = Solution::new(big_vec[..500_000].to_vec(), big_vec[500_000..].to_vec());
    println!("{}", solution.find_median_sorted_arrays());
}
