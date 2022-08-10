struct Solution;

impl Solution {
    fn parse(str: &str) -> (i32, i32) {
        str.split('+')
            .flat_map(|mem| {
                mem.split('-')
                    .enumerate()
                    .map(|(i, v)| if i == 0 { (1, v) } else { (-1, v) })
                    .filter(|(_, s)| !s.is_empty())
            })
            .map(|(signal, item)| {
                if item.ends_with('x') {
                    (
                        signal * item[..item.len() - 1].parse::<i32>().unwrap_or(1),
                        true,
                    )
                } else {
                    (signal * item.parse::<i32>().unwrap(), false)
                }
            })
            .fold(
                (0, 0),
                |(x, c), (i, v)| {
                    if v {
                        (x + i, c)
                    } else {
                        (x, c + i)
                    }
                },
            )
    }

    pub fn solve_equation(equation: String) -> String {
        let eq = equation.split('=').map(Self::parse).collect::<Vec<_>>();

        let ((lx, lc), (rx, rc)) = (eq[0], eq[1]);
        let x = lx - rx;
        let c = rc - lc;

        if x == 0 {
            (if c == 0 {
                "Infinite solutions"
            } else {
                "No solution"
            })
            .to_string()
        } else {
            format!("x={}", c / x)
        }
    }
}

fn main() {
    println!("{}", Solution::solve_equation("x+5-3+x=6+x-2".to_string()))
}
