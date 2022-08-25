use std::ops::Range;

struct Solution;

impl Solution {
    fn find(arr: &[i32], x: i32, range: Range<i32>) -> i32 {
        match range.len() {
            0 => range.end,
            1 => {
                let start = range.start;
                let end = range.end;
                if arr[start as usize] >= x {
                    start
                } else {
                    end
                }
            }
            _ => {
                let start = range.start;
                let end = range.end;
                let mid = (start + end) / 2;
                if arr[mid as usize] >= x {
                    Self::find(arr, x, start..mid)
                } else {
                    Self::find(arr, x, mid..end)
                }
            }
        }
    }

    pub fn find_closest_elements(arr: Vec<i32>, k: i32, x: i32) -> Vec<i32> {
        let len = arr.len() as i32;
        let mut j = Self::find(&arr, x, 0..len);
        let mut i = j - 1;
        for _ in 0..k {
            if i < 0 {
                j += 1;
                continue;
            }
            if j >= len {
                i -= 1;
                continue;
            }
            if (arr[i as usize] - x).abs() <= (arr[j as usize] - x).abs() {
                i -= 1;
            } else {
                j += 1;
            }
        }
        arr[(i + 1) as usize..j as usize].iter().copied().collect()
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::find_closest_elements(vec![1, 2, 2, 3, 4, 4, 4, 5, 7, 8, 8, 9], 3, 6)
    )
}
