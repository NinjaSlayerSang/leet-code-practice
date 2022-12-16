struct Solution;

impl Solution {
    fn root(parents: &mut Vec<usize>, i: usize) -> usize {
        if parents[i] != i {
            parents[i] = Self::root(parents, parents[i]);
        }
        parents[i]
    }

    fn merge(parents: &mut Vec<usize>, a: usize, b: usize) {
        let ra = Self::root(parents, a);
        let rb = Self::root(parents, b);
        parents[rb] = ra;
    }

    fn inquire(parents: &mut Vec<usize>, a: usize, b: usize) -> bool {
        let ra = Self::root(parents, a);
        let rb = Self::root(parents, b);
        ra == rb
    }

    pub fn distance_limited_paths_exist(
        n: i32,
        mut edge_list: Vec<Vec<i32>>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        let n = n as usize;
        let mut parents = (0..n).into_iter().collect::<Vec<_>>();
        edge_list.sort_by(|a, b| a[2].cmp(&b[2]));
        let mut queries = queries.into_iter().enumerate().collect::<Vec<_>>();
        queries.sort_by(|a, b| a.1[2].cmp(&b.1[2]));

        let mut edge_cursor = 0;
        let mut result = vec![false; queries.len()];
        for (query_index, query) in queries {
            while edge_cursor < edge_list.len() && edge_list[edge_cursor][2] < query[2] {
                Self::merge(
                    &mut parents,
                    edge_list[edge_cursor][0] as usize,
                    edge_list[edge_cursor][1] as usize,
                );
                edge_cursor += 1;
            }
            result[query_index] = Self::inquire(&mut parents, query[0] as usize, query[1] as usize);
        }
        result
    }
}

fn main() {
    println!(
        "{:?}",
        Solution::distance_limited_paths_exist(
            5,
            [[0, 1, 10], [1, 2, 5], [2, 3, 9], [3, 4, 13]]
                .map(|v| v.to_vec())
                .to_vec(),
            [[0, 4, 14], [1, 4, 13]].map(|v| v.to_vec()).to_vec()
        )
    );
}
