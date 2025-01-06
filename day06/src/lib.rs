pub fn process_numbers(numbers: &[i32]) -> i32 {
     numbers.iter().filter(|&x| x % 3 == 0).map(|x|x * 2).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_numbers() {
        let input = vec![1, 2, 3, 4, 5, 6];
        let output = process_numbers(&input);
        assert_eq!(output, 18)
    }

    #[test]
    fn test_process_numbers_only_divide_by_3() {
        let input = vec![0, 9, 15];
        let output = process_numbers(&input);
        assert_eq!(output, 48);
    }
}
