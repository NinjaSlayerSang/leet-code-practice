struct Solution;

impl Solution {
    pub fn max_chunks_to_sorted(arr: Vec<i32>) -> i32 {
        let len = arr.len();

        let mut m = i32::MIN;
        let prefix_max = arr
            .iter()
            .map(|i| {
                let r = m;
                m = m.max(*i);
                r
            })
            .collect::<Vec<_>>();

        m = i32::MAX;
        let mut suffix_min = arr
            .into_iter()
            .rev()
            .map(|i| {
                m = m.min(i);
                m
            })
            .collect::<Vec<_>>();
        suffix_min.reverse();

        (0..len).fold(0, |r, i| r + (prefix_max[i] <= suffix_min[i]) as i32)
    }
}

fn main() {
    println!("{}", Solution::max_chunks_to_sorted(vec![2, 1, 3, 4, 4]))
}
