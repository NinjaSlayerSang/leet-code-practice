struct Solution;

trait IsSubSequence {
    fn is_sub_sequence(&self, target: &str) -> bool;
}

impl IsSubSequence for str {
    fn is_sub_sequence(&self, target: &str) -> bool {
        fn calc(s: &str, t: &str, m: usize, n: usize) -> bool {
            if m == 0 {
                return true;
            };
            if m > n {
                return false;
            };
            if s.chars().nth(m - 1) == t.chars().nth(n - 1) {
                calc(s, t, m - 1, n - 1) || calc(s, t, m, n - 1)
            } else {
                calc(s, t, m, n - 1)
            }
        }
        calc(self, target, self.len(), target.len())
    }
}

impl Solution {
    pub fn find_lu_slength(strs: Vec<String>) -> i32 {
        use std::collections::HashMap;
        let mut m = HashMap::<String, i32>::with_capacity(strs.len());
        for k in strs {
            *m.entry(k).or_insert(0) += 1;
        }
        let mut s = m.into_iter().collect::<Vec<_>>();
        s.sort_by(|(a, _), (b, _)| b.len().cmp(&a.len()));
        'i: for (i, (k, c)) in s.iter().enumerate() {
            if *c > 1 {
                continue 'i;
            };
            let mut flag = true;
            'j: for (j, (l, _)) in s.iter().enumerate() {
                if i == j {
                    continue 'j;
                }
                if k.len() >= l.len() {
                    break 'j;
                }
                if k.is_sub_sequence(l) {
                    flag = false;
                    break 'j;
                }
            }
            if flag {
                return k.len() as i32;
            }
        }
        -1
    }
}

fn main() {
    let input = (vec!["aabbcc", "aabbcc", "b", "bc"])
        .into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<_>>();
    let output = Solution::find_lu_slength(input);
    print!("{}", output);
}
