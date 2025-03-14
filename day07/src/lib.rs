/// process_and_sum
/// 1. remove all negative numbers
/// 2. double the rest
/// 3. sum all numbers, that are devidable by 5
pub fn process_and_sum(numbers: &[i32]) -> i32 {
    numbers
        .iter()
        .filter(|x| x.is_positive())
        .map(|n| n * 2)
        .filter(|e| e % 5 == 0)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_and_sum() {
        let input = vec![1, -2, 3, 4, 5, -10, 15];
        let output = process_and_sum(&input);
        assert_eq!(40, output);
    }
}
