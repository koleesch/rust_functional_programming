pub fn squares(numbers: &[i32]) -> Vec<i32> {
    numbers.iter().map(|x| x * x).collect()
}

pub fn filter_odd(numbers: &[i32]) -> Vec<i32> {
    numbers.iter().copied().filter(|x| x % 2 != 0).collect()
}

pub fn sum(numbers: &[i32]) -> i32 {
    numbers.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_squares() {
        let input = vec![1, 2, 3, 4, 5];
        let output = squares(&input);
        assert_eq!(output, vec![1, 4, 9, 16, 25]);
    }

    #[test]
    fn test_filter_odd() {
        let input = vec![1, 2, 3, 4, 5];
        let output = filter_odd(&input);
        assert_eq!(output, vec![1, 3, 5]);
    }

    #[test]
    fn test_filter_empty() {
        let input = vec![];
        let output = filter_odd(&input);
        assert_eq!(output, vec![]);
    }

    #[test]
    fn test_filter_no_odds() {
        let input = vec![0, 2, 4, 6, 8];
        let output = filter_odd(&input);
        assert_eq!(output, vec![]);
    }

    #[test]
    fn test_filter_only_odds() {
        let input = vec![1, 3, 5, 7, 9];
        let output = filter_odd(&input);
        assert_eq!(output, vec![1, 3, 5, 7, 9]);
    }

    #[test]
    fn test_sum() {
        let input = vec![1, 2, 3, 4, 5];
        let output = sum(&input);
        assert_eq!(output, 15);
    }
}
