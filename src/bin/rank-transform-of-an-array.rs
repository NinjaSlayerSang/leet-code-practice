struct Solution;

impl Solution {
    pub fn array_rank_transform(arr: Vec<i32>) -> Vec<i32> {
        struct Item {
            index: usize,
            value: i32,
            number: i32,
        }

        let mut items = arr
            .into_iter()
            .enumerate()
            .map(|(index, value)| Item {
                index,
                value,
                number: 0,
            })
            .collect::<Vec<_>>();
        items.sort_by(|a, b| a.value.cmp(&b.value));

        let mut number = 0;
        let mut current = i32::MIN;
        items.iter_mut().for_each(|item| {
            if item.value > current {
                current = item.value;
                number += 1;
            }
            item.number = number
        });

        items.sort_by(|a, b| a.index.cmp(&b.index));

        items
            .into_iter()
            .map(|item| item.number)
            .collect::<Vec<_>>()
    }
}

fn main() {
    let arr = vec![37, 12, 28, 9, 100, 56, 80, 5, 12];

    let result = Solution::array_rank_transform(arr);

    println!("{:?}", result)
}
