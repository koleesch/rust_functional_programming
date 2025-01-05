pub fn factorial_tail_recursion(n: f64, accumulator: f64) -> f64 {
    if n <= 0.0 {
        return accumulator;
    }

    if accumulator <= 0.0 {
       return factorial_tail_recursion(n, 1.0)
    }

    factorial_tail_recursion(n - 1.0, n * accumulator)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_factorial_tail_recursion() {
        let result = factorial_tail_recursion(5.0, 1.0);
        assert_eq!(result, 120.0);
    }
    #[test]
    fn test_factorial_tail_recursion_accumulator_0() {
        let result = factorial_tail_recursion(5.0, 0.0);
        assert_eq!(result, 120.0);
    }
    #[test]
    fn test_factorial_tail_recursion_n_zero() {
        let result = factorial_tail_recursion(0.0, 0.0);
        assert_eq!(result, 0.0);
    }

    #[test]
    fn test_factorial_tail_recursion_large_number() {
        let result = factorial_tail_recursion(20.0, 1.0);
        assert!((result - 2_432_902_008_176_640.0).abs() < 1e-6); // Näherungstoleranz
    }
}
