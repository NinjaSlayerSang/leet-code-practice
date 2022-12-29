struct Solution;

impl Solution {
    fn binary_search(range: (i32, i32), predicate: &dyn Fn(i32) -> bool) -> i32 {
        if range.0 >= range.1 {
            range.1
        } else {
            let mid = (range.0 + range.1) / 2;
            if predicate(mid) {
                Self::binary_search((range.0, mid), predicate)
            } else {
                Self::binary_search((mid + 1, range.1), predicate)
            }
        }
    }

    pub fn minimum_size(nums: Vec<i32>, max_operations: i32) -> i32 {
        Self::binary_search((1, nums.iter().max().copied().unwrap()), &|k| {
            nums.iter().fold(0, |c, &i| c + i / k - (i % k == 0) as i32) <= max_operations
        })
    }
}

fn main() {
    println!("{}", Solution::minimum_size(vec![2, 4, 8, 2], 4));
}
