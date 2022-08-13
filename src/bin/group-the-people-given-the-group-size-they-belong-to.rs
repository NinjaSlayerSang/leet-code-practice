struct Solution;

impl Solution {
    pub fn group_the_people(group_sizes: Vec<i32>) -> Vec<Vec<i32>> {
        use std::collections::HashMap;

        let mut result = Vec::<Vec<i32>>::new();
        let mut groups = HashMap::<i32, Vec<i32>>::new();

        for (i, size) in group_sizes.into_iter().enumerate() {
            if let Some(group) = groups.get_mut(&size) {
                group.push(i as i32);
            } else {
                let mut new_group = Vec::<i32>::with_capacity(size as usize);
                new_group.push(i as i32);
                groups.insert(size, new_group);
            }

            if groups.get(&size).unwrap().len() == size as usize {
                let group = groups.remove(&size).unwrap();
                result.push(group);
            }
        }

        result
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::group_the_people(vec![3, 3, 3, 3, 3, 1, 3])
    )
}
