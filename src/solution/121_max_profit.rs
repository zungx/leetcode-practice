
pub struct Solution {}

impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min_price = prices[0];
        let mut max = 0;
        for idx in 1..prices.len() {
            max = i32::max(max, prices[idx] - min_price);
            min_price = i32::min(min_price, prices[idx])
        }
        max
    }
}

fn main() {   
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_121() {
        assert_eq!(Solution::max_profit(vec![7, 1, 5, 3, 6, 4]), 5);
        assert_eq!(Solution::max_profit(vec![7, 6, 4, 3, 1]), 0);
    }
}