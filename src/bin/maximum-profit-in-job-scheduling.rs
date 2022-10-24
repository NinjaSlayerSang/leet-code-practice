struct Solution;

struct Job {
    start_time: i32,
    end_time: i32,
    profit: i32,
}

impl Solution {
    pub fn job_scheduling(start_time: Vec<i32>, end_time: Vec<i32>, profit: Vec<i32>) -> i32 {
        use std::collections::BTreeMap;

        let len = profit.len();
        let mut jobs = (0..len)
            .into_iter()
            .map(|i| Job {
                start_time: start_time[i],
                end_time: end_time[i],
                profit: profit[i],
            })
            .collect::<Vec<_>>();
        jobs.sort_by(|a, b| {
            a.end_time
                .cmp(&b.end_time)
                .then(a.start_time.cmp(&b.start_time))
                .then(a.profit.cmp(&b.profit))
        });

        let mut f = vec![0; len];
        let mut b_tree = BTreeMap::<i32, Vec<usize>>::new();

        for j in 0..len {
            f[j] = jobs[j].profit
                + b_tree
                    .iter()
                    .rev()
                    .find(|(_, l)| l.iter().any(|&i| jobs[i].end_time <= jobs[j].start_time))
                    .map_or(0, |(&p, _)| p);
            b_tree.entry(f[j]).or_default().push(j);
        }

        f.into_iter().max().unwrap_or(0)
    }
}

fn main() {
    let start_time = vec![1, 2, 3, 4, 6];
    let end_time = vec![3, 5, 10, 6, 9];
    let profit = vec![20, 20, 100, 70, 60];
    println!("{}", Solution::job_scheduling(start_time, end_time, profit));
}
