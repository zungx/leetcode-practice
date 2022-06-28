use std::collections::HashMap;
pub struct Solution {}

impl Solution {

    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut letters = HashMap::new();
        for c in magazine.chars() {
            let count = letters.entry(c).or_insert(0);
            *count += 1;
        }
        for c in ransom_note.chars() {
            if let Some(count) = letters.get_mut(&c) {
                *count -= 1;
                if *count < 0 {
                    return false;
                }
            } else {
                return false;
            }
        }
        true
    }
}


fn main() {
    Solution::can_construct("aa".to_string(), "aab".to_string());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_383() {
        assert_eq!(Solution::can_construct("aa".to_string(), "aab".to_string()), true);
        assert_eq!(Solution::can_construct("aa".to_string(), "ab".to_string()), false);
        assert_eq!(Solution::can_construct("a".to_string(), "b".to_string()), false);
    }
}