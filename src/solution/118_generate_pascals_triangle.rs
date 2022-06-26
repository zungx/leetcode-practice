
pub struct Solution {}

impl Solution {

    pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        if num_rows <= 0 {
            return result;
        }

        let mut curr = vec![1];
        for _ in 0..num_rows {
            let mut next = vec![1; curr.len() + 1];
            for i in 1..curr.len() {
                next[i] = curr[i -1] + curr[i];
            }
            result.push(curr);
            curr = next;
        }
        
        result
    }
}


fn main() {
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_118() {
        assert_eq!(Solution::generate(1), vec![vec![1]]);
        assert_eq!(
            Solution::generate(5),
            vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1]
            ]
        );
    }
}