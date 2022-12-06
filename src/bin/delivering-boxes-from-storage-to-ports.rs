struct Solution;

impl Solution {
    pub fn box_delivering(
        boxes: Vec<Vec<i32>>,
        ports_count: i32,
        max_boxes: i32,
        max_weight: i32,
    ) -> i32 {
        use std::collections::VecDeque;

        let n = boxes.len();
        let mut c = vec![0; n + 1];
        let mut s = vec![0; n + 1];
        let mut f = vec![0; n + 1];

        for i in 1..n {
            c[i] = c[i - 1] + (boxes[i][0] != boxes[i - 1][0]) as i32;
        }
        c[n] = c[n - 1] + 1;

        for i in 1..=n {
            s[i] = s[i - 1] + boxes[i - 1][1];
        }

        let mut deq = VecDeque::from([0]);

        for i in 1..=n {
            while let Some(j) = deq.front().copied() {
                if i - j <= max_boxes as usize && s[i] - s[j] <= max_weight {
                    f[i] = f[j] + c[i - 1] - c[j] + 2;
                    break;
                } else {
                    deq.pop_front();
                }
            }
            while let Some(j) = deq.back().copied() {
                if f[j] - c[j] >= f[i] - c[i] {
                    deq.pop_back();
                } else {
                    break;
                }
            }
            deq.push_back(i);
        }

        f[n]
    }
}

fn main() {
    println!(
        "{}",
        Solution::box_delivering(
            [
                [2, 4],
                [2, 5],
                [3, 1],
                [3, 2],
                [3, 7],
                [3, 1],
                [4, 4],
                [1, 3],
                [5, 2]
            ]
            .map(|v| v.to_vec())
            .to_vec(),
            5,
            5,
            7
        )
    );
}
