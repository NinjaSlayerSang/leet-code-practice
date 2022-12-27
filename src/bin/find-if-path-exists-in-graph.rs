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

    fn query(parents: &mut Vec<usize>, a: usize, b: usize) -> bool {
        let ra = Self::root(parents, a);
        let rb = Self::root(parents, b);
        ra == rb
    }

    pub fn valid_path(n: i32, edges: Vec<Vec<i32>>, source: i32, destination: i32) -> bool {
        let mut parents = (0..n as usize).into_iter().collect::<Vec<_>>();

        for edge in edges {
            Self::merge(&mut parents, edge[0] as usize, edge[1] as usize);
        }

        Self::query(&mut parents, source as usize, destination as usize)
    }
}

fn main() {
    println!(
        "{}",
        Solution::valid_path(
            6,
            [[0, 1], [0, 2], [3, 5], [5, 4], [4, 3]]
                .map(|v| v.to_vec())
                .to_vec(),
            0,
            5
        )
    );
}
