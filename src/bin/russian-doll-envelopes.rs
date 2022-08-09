use std::ops::{Index, Range};

struct Solution;

trait Extension
where
    Self: Index<usize, Output = Self::Item>,
{
    type Item: Ord;

    fn find_last_pred(&self, range: Range<isize>, item: &Self::Item) -> isize {
        use std::cmp::Ordering::{Equal, Greater, Less};

        match range.len() {
            0 => range.start - 1,
            1 => match self[range.start as usize].cmp(item) {
                Less => range.start,
                Equal | Greater => range.start - 1,
            },
            _ => {
                let mid = (range.start + range.end) / 2;
                match self[mid as usize].cmp(item) {
                    Greater | Equal => Self::find_last_pred(self, range.start..mid, item),
                    Less => Self::find_last_pred(self, mid..range.end, item),
                }
            }
        }
    }
}

impl Extension for Vec<i32> {
    type Item = i32;
}

impl Solution {
    pub fn max_envelopes(mut envelopes: Vec<Vec<i32>>) -> i32 {
        let len = envelopes.len();
        envelopes.sort_by(|a, b| a[0].cmp(&b[0]).then(b[1].cmp(&a[1])));

        let mut f = Vec::<i32>::with_capacity(len);

        let mut update = |value: i32| {
            let fl = f.len() as isize;
            let i = f.find_last_pred(0..fl, &value) + 1;
            if i < fl {
                f[i as usize] = value
            } else {
                f.push(value)
            }
        };

        for i in 0..len {
            update(envelopes[i][1])
        }

        f.len() as i32
    }
}

fn main() {
    println!(
        "{}",
        Solution::max_envelopes(vec![vec![5, 4], vec![6, 4], vec![6, 7], vec![2, 3]])
    )
}
