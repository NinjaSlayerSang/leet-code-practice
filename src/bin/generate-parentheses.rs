struct Solution;

impl Solution {
    fn dfs(n: usize, p: usize, record: &mut Vec<char>, l: usize, result: &mut Vec<String>) {
        if p < 2 * n {
            if l < n {
                record[p] = '(';
                Self::dfs(n, p + 1, record, l + 1, result);
            }
            if p < 2 * l {
                record[p] = ')';
                Self::dfs(n, p + 1, record, l, result);
            }
        } else {
            result.push(record.iter().collect());
        }
    }

    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let n = n as usize;
        let mut record = vec!['*'; 2 * n];
        let mut result = Vec::<String>::new();
        Self::dfs(n, 0, &mut record, 0, &mut result);
        result
    }
}

fn main() {
    println!("{:?}", Solution::generate_parenthesis(3));
}
