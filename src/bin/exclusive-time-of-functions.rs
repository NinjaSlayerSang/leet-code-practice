struct Log {
    id: usize,
    signal: bool,
    timestamp: i32,
}

struct Solution;

impl Solution {
    pub fn exclusive_time(n: i32, logs: Vec<String>) -> Vec<i32> {
        let n = n as usize;
        let mut logs = logs
            .into_iter()
            .map(|log| {
                let log = log.split(':').into_iter().collect::<Vec<_>>();
                Log {
                    id: log[0].parse().unwrap(),
                    signal: log[1] == "start",
                    timestamp: log[2].parse().unwrap(),
                }
            })
            .collect::<Vec<_>>();

        logs.sort_by(|a, b| a.timestamp.cmp(&b.timestamp).then(b.signal.cmp(&a.signal)));

        let mut time = vec![0; n];
        let mut stack = Vec::<usize>::new();
        let mut clock = 0;

        for log in logs {
            if log.signal {
                if let Some(top) = stack.last().copied() {
                    time[top] += log.timestamp - clock;
                }
                stack.push(log.id);
                clock = log.timestamp;
            } else {
                let top = stack.pop().unwrap();
                assert_eq!(top, log.id);
                time[top] += log.timestamp - clock + 1;
                clock = log.timestamp + 1;
            }
        }

        time
    }
}

fn main() {
    let n = 2;
    let logs = vec![
        "0:start:0",
        "0:start:2",
        "0:end:5",
        "1:start:7",
        "1:end:7",
        "0:end:8",
    ];
    println!(
        "{:?}",
        Solution::exclusive_time(n, logs.into_iter().map(|s| s.to_string()).collect())
    )
}
