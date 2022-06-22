
pub struct Solution {}

impl Solution {
    pub fn intersect(nums1: Vec<i32>, mut nums2: Vec<i32>) -> Vec<i32> {
        let mut vec: Vec<i32> = Vec::new();
        for n in nums1.iter() {
            if nums2.contains(&n) {
                vec.push(*n);
                nums2.remove(nums2.iter().position(|&x| x == *n).unwrap());
            }
        }
        vec
    }
}

fn main() {   
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_350() {
        assert_eq!(Solution::intersect(vec![1,2,2,1], vec![2,2]), vec![2,2]);
        assert_eq!(Solution::intersect(vec![4,9,5], vec![9,4,9,8,4]), vec![4,9]);
        assert_eq!(Solution::intersect(vec![1,2,2,1], vec![2]), vec![2]);
        assert_eq!(Solution::intersect(vec![3,1,2], vec![1,1]), vec![1]);
    }
}