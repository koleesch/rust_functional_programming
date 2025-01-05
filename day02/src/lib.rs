/// filter_by_divisor
pub fn filter_by_divisor(numbers: &[i32], divisor: i32) -> Option<Vec<i32>> {
    // Divison durch 0 ist nicht möglich
    if divisor == 0 {
        return None;
    }

    // filtern
    // 1. Iterator auf Vektor erzeugen
    // 2. für jedes Element prüfen, ob eine Modulo-Operation ohne Rest möglich ist
    // 3. Kopie der gefilterten Datensätze erzeugen
    // 4. Einsammeln der gefilterten Datensätze
    let filtered: Vec<i32> = numbers
        .iter()
        .filter(|&&x| x % divisor == 0)
        .copied()
        .collect();

    Some(filtered)
}

/// transform_and_filter
pub fn transform_and_filter(numbers: &[i32], divisor: i32) -> Option<Vec<i32>> {
    // 1. Iterator auf Verktor erzeugen
    // 2. aller Werte zum Quadrat via map
    // 3. einsammeln der Werte
    let transformed: Vec<i32> = numbers.iter().map(|&x| x * x).collect();

    filter_by_divisor(&transformed, divisor).filter(|filtered| !filtered.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_filter_by_devisor_2() {
        let input = vec![1, 2, 3, 4, 5];
        let output = filter_by_divisor(&input, 2).unwrap();
        assert_eq!(output, vec![2, 4]);
    }
    #[test]
    fn test_filter_by_invalid_devisor() {
        let input = vec![1, 2, 3, 4, 5];
        let divisor = 0;
        let output = filter_by_divisor(&input, divisor);
        assert_eq!(output, None);
    }
    #[test]
    fn test_filter_no_matches() {
        let input: Vec<i32> = vec![1, 2, 3, 4, 5];
        let divisor = 7;
        let output = filter_by_divisor(&input, divisor).unwrap();
        assert_eq!(output, vec![]);
    }

    #[test]
    fn test_transform_and_filter_by_devisor_2() {
        let input = vec![1, 2, 3, 4, 5];
        let output = transform_and_filter(&input, 2).unwrap();
        assert_eq!(output, vec![4, 16]);
    }
    #[test]
    fn test_transform_and_filter_by_invalid_devisor() {
        let input = vec![1, 2, 3, 4, 5];
        let divisor = 0;
        let output = transform_and_filter(&input, divisor);
        assert_eq!(output, None);
    }
    #[test]
    fn test_transform_and_filter_no_matches() {
        let input: Vec<i32> = vec![1, 2, 3, 4, 5];
        let divisor = 7;
        let output = transform_and_filter(&input, divisor);
        assert_eq!(output, None);
    }
}
