pub fn diagonal_difference(arr: &[Vec<i32>]) -> i32 {
    let n = arr.len();
    let mut left = 0;
    let mut right = 0;

    for i in 0..n {
        left += arr[i][i];
        right += arr[i][n - 1 - i];
    }

    (left - right).abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagonal_difference() {
        let arr = vec![
            vec![11, 2, 4],
            vec![4, 5, 6],
            vec![10, 8, -12],
        ];

        assert_eq!(diagonal_difference(&arr), 15);
    }
}