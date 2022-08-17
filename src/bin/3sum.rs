struct Solution;

impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        use std::collections::{HashMap, HashSet};

        let mut stat = HashMap::<i32, usize>::new();
        for k in nums {
            *stat.entry(k).or_default() += 1;
        }
        let stat_queue = stat.iter().collect::<Vec<_>>();

        let mut sum_map = HashMap::<i32, Vec<[i32; 2]>>::new();
        for i in 0..stat_queue.len() {
            for j in i..stat_queue.len() {
                if i != j || *stat_queue[j].1 > 1 {
                    let a = *stat_queue[i].0;
                    let b = *stat_queue[j].0;
                    let t = -(a + b);
                    sum_map.entry(t).or_default().push([a, b]);
                }
            }
        }

        let mut result = HashSet::<[i32; 3]>::new();
        for (c, cn) in stat_queue {
            if let Some(v) = sum_map.get(c) {
                for [a, b] in v {
                    if match (c.eq(a), c.eq(b)) {
                        (true, true) => *cn >= 3,
                        (true, false) | (false, true) => *cn >= 2,
                        (false, false) => true,
                    } {
                        let mut l = [*a, *b, *c];
                        l.sort();
                        result.insert(l);
                    }
                }
            }
        }

        result.into_iter().map(Vec::from).collect()
    }
}

fn main() {
    println!("{:?}", Solution::three_sum(vec![-1, 0, 1, 2, -1, -4]))
}
