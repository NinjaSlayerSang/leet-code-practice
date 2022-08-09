struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        use std::collections::HashMap;

        let mut hash = HashMap::<i32, Vec<usize>>::new();
        nums.iter().enumerate().for_each(|(i, k)| {
            if hash.contains_key(k) {
                let list = hash.get_mut(k).unwrap();
                list.push(i);
            } else {
                hash.insert(*k, vec![i]);
            }
        });

        for (i, k) in nums.into_iter().enumerate() {
            let t = target - k;
            if let Some(list) = hash.get(&t) {
                for j in list {
                    if i != *j {
                        return vec![i as i32, *j as i32];
                    }
                }
            }
        }

        panic!()
    }
}

fn main() {
    println!("{:?}", Solution::two_sum(vec![3, 2, 4], 6))
}
