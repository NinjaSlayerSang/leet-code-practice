struct Solution;

impl Solution {
    pub fn can_choose(groups: Vec<Vec<i32>>, nums: Vec<i32>) -> bool {
        let nexts = groups
            .iter()
            .map(|v| {
                let mut next = vec![-1; v.len() + 1];
                for i in 0..v.len() {
                    let mut p = next[i];
                    while p >= 0 {
                        if v[i] == v[p as usize] {
                            break;
                        } else {
                            p = next[p as usize];
                        }
                    }
                    next[i + 1] = p + 1;
                }
                next
            })
            .collect::<Vec<_>>();

        let mut k = 0;
        let mut j = 0;
        for i in 0..nums.len() {
            while j >= 0 {
                if nums[i] == groups[k][j as usize] {
                    break;
                } else {
                    j = nexts[k][j as usize]
                }
            }
            j += 1;
            if j as usize >= groups[k].len() {
                k += 1;
                j = 0;
            }
            if k >= groups.len() {
                return true;
            }
        }

        false
    }
}

fn main() {
    println!(
        "{}",
        Solution::can_choose(
            vec![vec![1, 2, 3], vec![3, 4]],
            vec![7, 7, 1, 2, 3, 4, 7, 7]
        )
    );
}
