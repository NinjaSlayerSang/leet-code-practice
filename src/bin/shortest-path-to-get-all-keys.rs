struct Solution;

use std::collections::HashSet;

const DIRS: [(i32, i32); 4] = [(0, 1), (1, 0), (0, -1), (-1, 0)];

impl Solution {
    pub fn shortest_path_all_keys(grid: Vec<String>) -> i32 {
        let m = grid.len();
        let n = grid[0].len();
        let map = grid
            .into_iter()
            .map(|s| s.chars().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let mut stack = Vec::<(usize, usize, usize, HashSet<char>)>::new();
        let mut mem = vec![vec![Vec::<HashSet<char>>::new(); n]; m];
        let mut all_keys = HashSet::<char>::new();

        for i in 0..m {
            for j in 0..n {
                let c = map[i][j];
                match c {
                    '@' => {
                        stack.push((i, j, 0, HashSet::new()));
                        mem[i][j].push(HashSet::new());
                    }
                    'a'..='z' => {
                        all_keys.insert(c);
                    }
                    _ => {}
                }
            }
        }

        let mut p = 0;
        let mut r = -1;

        'bfs: while p < stack.len() {
            let (i, j, step, keys) = stack[p].clone();
            for (di, dj) in DIRS.into_iter() {
                let x = i as i32 + di;
                let y = j as i32 + dj;
                if (0..m as i32).contains(&x) && (0..n as i32).contains(&y) {
                    let (x, y) = (x as usize, y as usize);
                    let c = map[x][y];
                    let mut try_insert = |keys: HashSet<char>| {
                        if !mem[x][y].contains(&keys) {
                            stack.push((x, y, step + 1, keys.clone()));
                            mem[x][y].push(keys);
                        }
                    };
                    match c {
                        '.' | '@' => try_insert(keys.clone()),
                        'a'..='z' => {
                            let mut new_keys = keys.clone();
                            new_keys.insert(c);
                            if all_keys.eq(&new_keys) {
                                r = step as i32 + 1;
                                break 'bfs;
                            }
                            try_insert(new_keys);
                        }
                        'A'..='Z' => {
                            if keys.contains(&c.to_ascii_lowercase()) {
                                try_insert(keys.clone());
                            }
                        }
                        _ => {}
                    }
                }
            }
            p += 1;
        }

        r
    }
}

fn main() {
    println!(
        "{}",
        Solution::shortest_path_all_keys(
            vec!["@..aA", "..B#.", "....b"]
                .into_iter()
                .map(|s| s.to_string())
                .collect()
        )
    )
}
