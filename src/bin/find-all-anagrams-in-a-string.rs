struct AlphaBucket([usize; 26]);

impl PartialEq for AlphaBucket {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..26 {
            if self.0[i] != other.0[i] {
                return false;
            }
        }
        true
    }
}

impl AlphaBucket {
    fn char_to_index(ch: char) -> usize {
        ch as usize - 'a' as usize
    }

    pub fn new() -> Self {
        Self([0usize; 26])
    }

    pub fn push(&mut self, ch: char) {
        self.0[Self::char_to_index(ch)] += 1
    }

    pub fn pop(&mut self, ch: char) {
        self.0[Self::char_to_index(ch)] -= 1
    }
}

struct Solution;

impl Solution {
    pub fn find_anagrams(s: String, p: String) -> Vec<i32> {
        let mut result = Vec::<i32>::new();

        if s.len() >= p.len() {
            let mut p_bucket = AlphaBucket::new();
            p.chars().for_each(|ch| p_bucket.push(ch));

            let mut s_bucket = AlphaBucket::new();
            let s = s.chars().collect::<Vec<_>>();
            s[0..p.len()].iter().for_each(|ch| s_bucket.push(*ch));
            if s_bucket == p_bucket {
                result.push(0)
            }

            for i in 1..s.len() - p.len() + 1 {
                s_bucket.push(s[i + p.len() - 1]);
                s_bucket.pop(s[i - 1]);
                if s_bucket == p_bucket {
                    result.push(i as i32)
                }
            }
        }

        result
    }
}

fn main() {
    let s = "cbaebabacd";
    let p = "abc";
    println!(
        "{:?}",
        Solution::find_anagrams(s.to_string(), p.to_string())
    );
}
