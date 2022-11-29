struct Solution;

impl Solution {
    pub fn reachable_nodes(edges: Vec<Vec<i32>>, max_moves: i32, n: i32) -> i32 {
        use std::collections::HashMap;

        let n = n as usize;
        let mut map = HashMap::<usize, Vec<(usize, i32)>>::new();
        for edge in edges.iter() {
            map.entry(edge[0] as usize)
                .or_default()
                .push((edge[1] as usize, edge[2]));
            map.entry(edge[1] as usize)
                .or_default()
                .push((edge[0] as usize, edge[2]));
        }

        let mut r = vec![i32::MIN; n];
        r[0] = max_moves;
        let mut stack = vec![0usize];
        let mut p = 0;

        while p < stack.len() {
            let i = stack[p];
            for (j, k) in map.get(&i).unwrap_or(&vec![]).iter().copied() {
                if r[j] < r[i] - k - 1 {
                    r[j] = r[i] - k - 1;
                    stack.push(j);
                }
            }
            p += 1;
        }

        r.iter().filter(|i| !i.is_negative()).count() as i32
            + edges.into_iter().fold(0, |c, edge| {
                c + edge[2].min(r[edge[0] as usize].max(0) + r[edge[1] as usize].max(0))
            })
    }
}

fn main() {
    println!(
        "{}",
        Solution::reachable_nodes(
            [[0, 1, 4], [1, 2, 6], [0, 2, 8], [1, 3, 1]]
                .map(|edge| edge.to_vec())
                .to_vec(),
            10,
            4
        )
    )
}
