struct Solution;

impl Solution {
    pub fn three_sum_closest(mut nums: Vec<i32>, target: i32) -> i32 {
        let len = nums.len();
        nums.sort();
        let mut delta = i32::MAX;
        let mut result = 0;
        'i: for i in 0..len - 2 {
            let mut j = i + 1;
            let mut k = len - 1;
            while j < k {
                let s = nums[i] + nums[j] + nums[k];
                let d = s - target;
                let ad = d.abs();
                if delta > ad {
                    delta = ad;
                    result = s;
                    if delta == 0 {
                        break 'i;
                    }
                }
                if d > 0 {
                    k -= 1;
                } else {
                    j += 1;
                }
            }
        }
        result
    }
}

fn main() {
    println!(
        "{}",
        Solution::three_sum_closest(vec![1, 2, 3, 4, 5, 6, 7, 8, 9], 23)
    )
}
