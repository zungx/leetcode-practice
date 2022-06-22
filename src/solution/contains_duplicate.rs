use std::collections::HashSet;

pub struct Solution {}

impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let mut exists = HashSet::new();
        for num in nums.iter() {
            if exists.contains(num) {
                return true;
            }
            exists.insert(num);
        }
        false
    }
}


fn main() {
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_217() {
        assert_eq!(Solution::contains_duplicate(vec![1]), false);
        assert_eq!(Solution::contains_duplicate(vec![]), false);
        assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 4]), false);
        assert_eq!(Solution::contains_duplicate(vec![1, 2, 3, 1]), true);
    }
}