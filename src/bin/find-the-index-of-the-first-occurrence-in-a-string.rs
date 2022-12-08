struct Solution;

impl Solution {
    pub fn str_str(haystack: String, needle: String) -> i32 {
        let haystack = haystack.chars().collect::<Vec<_>>();
        let needle = needle.chars().collect::<Vec<_>>();
        let (m, n) = (haystack.len(), needle.len());

        let mut next = vec![-1; n + 1];
        next[1] = 0;
        for i in 1..n {
            let mut p = next[i];
            while p >= 0 {
                if needle[i] == needle[p as usize] {
                    break;
                } else {
                    p = next[p as usize];
                }
            }
            next[i + 1] = p + 1;
        }

        let mut j = 0;
        for i in 0..m {
            while j >= 0 {
                if haystack[i] == needle[j as usize] {
                    break;
                } else {
                    j = next[j as usize];
                }
            }
            j += 1;
            if j as usize >= n {
                return i as i32 - j + 1;
            }
        }

        -1
    }
}

fn main() {
    println!(
        "{}",
        Solution::str_str("sadbutsad".to_string(), "but".to_string())
    )
}
