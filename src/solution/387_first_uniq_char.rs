
pub struct Solution {}

impl Solution {

    pub fn first_uniq_char(s: String) -> i32 {
        let mut m :std::collections::HashMap<char,i32> = std::collections::HashMap::new();
        for i in s.chars(){
            *m.entry(i).or_insert(0) += 1;
        }
       
        for (i,v) in s.chars().enumerate(){
            if m.get(&v).unwrap() == &1{
                return i as i32
            }   
        }
        -1
    }
}


fn main() {
    Solution::first_uniq_char("leetcode".to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_387() {
        assert_eq!(Solution::first_uniq_char("leetcode".to_string()),0);
        assert_eq!(Solution::first_uniq_char("loveleetcode".to_string()),2);
        assert_eq!(Solution::first_uniq_char("aabb".to_string()),-1);
    }
}