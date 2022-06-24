
pub struct Solution {}

impl Solution {

    pub fn matrix_reshape(mat: Vec<Vec<i32>>, r: i32, c: i32) -> Vec<Vec<i32>> {
        let rs = r as usize;
        let cs = c as usize;
        let m = mat.len();
        let n = mat[0].len();
        if m * n != cs * rs {
            return mat;
        }

        let mut result = vec![vec![0; cs]; rs];

        for idx in 0..(rs * cs) {
            result[idx / cs][idx % cs] = mat[idx / n][idx % n];
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
    fn test_566() {
        assert_eq!(vec![vec![1,2,3,4]], Solution::matrix_reshape(vec![vec![1,2], vec![3,4]], 1, 4));
        assert_eq!(vec![vec![1,2], vec![3,4]], Solution::matrix_reshape(vec![vec![1,2], vec![3,4]], 2, 2));   
    }
}