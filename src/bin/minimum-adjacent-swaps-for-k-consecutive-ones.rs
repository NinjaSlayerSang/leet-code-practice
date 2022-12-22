struct Solution;

impl Solution {
    fn prefix_sum<T: Copy + Default + std::ops::Add<Output = T>>(v: &Vec<T>) -> Vec<T> {
        let mut r = vec![T::default(); v.len() + 1];
        for i in 1..=v.len() {
            r[i] = r[i - 1] + v[i - 1];
        }
        r
    }

    pub fn min_moves(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut f = Vec::<i32>::new();
        let mut p = -1;
        for i in 0..nums.len() {
            if nums[i] == 1 {
                let i = i as i32;
                if p >= 0 {
                    f.push(i - p - 1);
                }
                p = i;
            }
        }
        let ff = Self::prefix_sum(&f);
        let fff = Self::prefix_sum(&ff);

        let l = f.len() + 2 - k;
        let h = k / 2;
        let m = k % 2;
        let mut r = vec![0; l];
        for i in 0..l {
            r[i] = fff[i] + fff[i + k] - fff[i + h] - fff[i + h + m];
        }

        r.into_iter().min().unwrap()
    }
}

fn main() {
    println!(
        "{}",
        Solution::min_moves(
            vec![
                1, 1, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0,
                1
            ],
            5
        )
    );
}
