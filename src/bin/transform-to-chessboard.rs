struct Solution;

impl Solution {
    fn diff(s: &Vec<i32>, v: &Vec<i32>) -> i32 {
        let len = s.len();
        let mut d = 0;
        for i in 0..len {
            if s[i] != v[i] {
                d += 1;
            }
        }
        d
    }

    fn trans(v: &Vec<i32>) -> i32 {
        let len = v.len();
        let mut c0 = 0i32;
        let mut c1 = 0i32;
        for i in v.iter() {
            if *i == 0 {
                c0 += 1;
            } else {
                c1 += 1;
            }
        }
        match c0 - c1 {
            -1 => {
                let s = (0..len).map(|i| ((i + 1) % 2) as i32).collect::<Vec<_>>();
                Self::diff(&s, v) / 2
            }
            0 => {
                let s0 = (0..len).map(|i| (i % 2) as i32).collect::<Vec<_>>();
                let s1 = (0..len).map(|i| ((i + 1) % 2) as i32).collect::<Vec<_>>();
                i32::min(Self::diff(&s0, v) / 2, Self::diff(&s1, v) / 2)
            }
            1 => {
                let s = (0..len).map(|i| (i % 2) as i32).collect::<Vec<_>>();
                Self::diff(&s, v) / 2
            }
            _ => -1,
        }
    }

    pub fn moves_to_chessboard(board: Vec<Vec<i32>>) -> i32 {
        let len = board.len();
        let r0 = &board[0];
        let tc = Self::trans(r0);
        if tc == -1 {
            return -1;
        }
        let mut rows = vec![0; len];
        for i in 0..len {
            let d = Self::diff(r0, &board[i]);
            if d == 0 {
                continue;
            }
            if d == len as i32 {
                rows[i] = 1;
                continue;
            }
            return -1;
        }
        let tr = Self::trans(&rows);
        if tr == -1 {
            return -1;
        }
        tc + tr
    }
}

fn main() {
    println!(
        "{}",
        Solution::moves_to_chessboard(vec![
            vec![0, 1, 1, 0],
            vec![0, 1, 1, 0],
            vec![1, 0, 0, 1],
            vec![1, 0, 0, 1]
        ])
    )
}
