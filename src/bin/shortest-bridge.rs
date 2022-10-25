struct Solution;

const DIR: [(i32, i32); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

impl Solution {
    fn spread(n: usize, point: (usize, usize)) -> Vec<(usize, usize)> {
        let range = 0..n as i32;
        let i = point.0 as i32;
        let j = point.1 as i32;
        DIR.iter()
            .filter_map(|(di, dj)| {
                let x = i + di;
                let y = j + dj;
                if range.contains(&x) && range.contains(&y) {
                    Some((x as usize, y as usize))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>()
    }

    fn find_first(grid: &Vec<Vec<i32>>, n: usize, v: i32) -> (usize, usize) {
        for i in 0..n {
            for j in 0..n {
                if grid[i][j] == v {
                    return (i, j);
                }
            }
        }
        (0, 0)
    }

    pub fn shortest_bridge(mut grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let mut s = Self::find_first(&grid, n, 1);

        let mut check = vec![vec![false; n]; n];
        let mut queue = vec![s];
        let mut p = 0;
        check[s.0][s.1] = true;

        while p < queue.len() {
            let now = queue[p];
            grid[now.0][now.1] = -1;
            for t in Self::spread(n, now) {
                if grid[t.0][t.1] == 1 && !check[t.0][t.1] {
                    queue.push(t);
                    check[t.0][t.1] = true;
                }
            }
            p += 1;
        }

        let mut result = vec![vec![i32::MAX; n]; n];
        s = Self::find_first(&grid, n, -1);
        queue = vec![s];
        p = 0;
        result[s.0][s.1] = 0;

        while p < queue.len() {
            let now = queue[p];
            for t in Self::spread(n, now) {
                let nr = if grid[t.0][t.1] == -1 {
                    0
                } else {
                    result[now.0][now.1] + 1
                };
                if nr < result[t.0][t.1] {
                    result[t.0][t.1] = nr;
                    queue.push(t);
                }
            }
            p += 1;
        }

        let mut m = i32::MAX;
        for i in 0..n {
            for j in 0..n {
                if grid[i][j] == 1 {
                    m = m.min(result[i][j]);
                }
            }
        }
        m - 1
    }
}

fn main() {
    let grid = vec![
        vec![1, 1, 1, 1, 1],
        vec![1, 0, 0, 0, 1],
        vec![1, 0, 1, 0, 1],
        vec![1, 0, 0, 0, 1],
        vec![1, 1, 1, 1, 1],
    ];
    println!("{}", Solution::shortest_bridge(grid));
}
