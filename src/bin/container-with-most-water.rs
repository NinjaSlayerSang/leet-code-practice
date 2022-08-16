struct Solution;

impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        use std::{
            cmp::Reverse,
            collections::{BinaryHeap, HashMap},
        };

        let len = height.len();
        let mut source = height.iter().copied().enumerate().collect::<Vec<_>>();
        source.sort_by(|a, b| b.1.cmp(&a.1));

        let mut min_heap = BinaryHeap::<Reverse<usize>>::with_capacity(len);
        let mut max_heap = BinaryHeap::<usize>::with_capacity(len);
        let mut range_map = HashMap::<i32, (usize, usize)>::new();
        let mut current = source[0].1;
        let mh = source.last().unwrap().1;
        for (i, h) in source {
            if h < current {
                range_map.insert(
                    current,
                    (
                        min_heap.peek().unwrap().0,
                        max_heap.peek().copied().unwrap(),
                    ),
                );
                current = h;
            }
            min_heap.push(Reverse(i));
            max_heap.push(i);
        }
        range_map.insert(
            mh,
            (
                min_heap.peek().unwrap().0,
                max_heap.peek().copied().unwrap(),
            ),
        );

        height.into_iter().enumerate().fold(0, |result, (i, h)| {
            let (l, r) = range_map.get(&h).copied().unwrap();
            result.max((i - l).max(r - i) as i32 * h)
        })
    }
}

fn main() {
    println!("{}", Solution::max_area(vec![1, 8, 6, 2, 5, 4, 8, 3, 7]))
}
