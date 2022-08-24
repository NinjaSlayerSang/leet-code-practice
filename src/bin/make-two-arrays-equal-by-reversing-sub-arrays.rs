struct Solution;

impl Solution {
    pub fn can_be_equal(mut target: Vec<i32>, mut arr: Vec<i32>) -> bool {
        target.len() == arr.len() && {
            target.sort();
            arr.sort();
            for i in 0..arr.len() {
                if target[i] != arr[i] {
                    return false;
                }
            }
            true
        }
    }
}

fn main() {
    println!(
        "{}",
        Solution::can_be_equal(vec![1, 2, 3, 4], vec![2, 4, 1, 3])
    )
}
