struct Solution;

impl Solution {
    pub fn closest_cost(base_costs: Vec<i32>, topping_costs: Vec<i32>, target: i32) -> i32 {
        use std::collections::HashSet;

        let mut set = base_costs.into_iter().collect::<HashSet<_>>();
        for i in topping_costs {
            let values = set.iter().copied().collect::<Vec<_>>();
            for v in values {
                set.insert(v + i);
                set.insert(v + 2 * i);
            }
        }

        let mut r = set
            .into_iter()
            .map(|v| ((v - target).abs(), v))
            .collect::<Vec<_>>();
        r.sort_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)));

        r[0].1
    }
}

fn main() {
    println!(
        "{}",
        Solution::closest_cost(vec![2, 3], vec![4, 5, 100], 18)
    );
}
