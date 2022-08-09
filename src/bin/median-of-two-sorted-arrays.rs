struct Solution;

impl Solution {
    fn find(a: &[i32], b: &[i32], k: usize) -> i32 {
        if a.len() == 0 {
            return b[k - 1];
        }

        if b.len() == 0 {
            return a[k - 1];
        }

        if k == 1 {
            return *a
                .first()
                .unwrap_or(&i32::MAX)
                .min(b.first().unwrap_or(&i32::MAX));
        }

        let mut i = k / 2;
        let mut j = k - i;
        if a.len() < i {
            j += i - a.len();
            i = a.len();
        }
        if b.len() < j {
            i += j - b.len();
            j = b.len();
        }

        if a[i - 1] <= b[j - 1] {
            Solution::find(&a[i..a.len()], b, k - i)
        } else {
            Solution::find(a, &b[j..b.len()], k - j)
        }
    }

    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let total = nums1.len() + nums2.len();
        if total % 2 == 0 {
            (Solution::find(&nums1, &nums2, total / 2) as f64
                + Solution::find(&nums1, &nums2, total / 2 + 1) as f64)
                / 2.0
        } else {
            Solution::find(&nums1, &nums2, total / 2 + 1) as f64
        }
    }
}

fn main() {
    println!(
        "{}",
        Solution::find_median_sorted_arrays(vec![1, 3, 5, 7, 9], vec![2, 4, 6, 8, 10])
    )
}
