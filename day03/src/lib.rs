///apply_and_filter
/// Nimmt eine Referenz auf eine Liste von Zahlen ('numbers') und eine Closure
/// Wendet die Closure auf jedes Element an.
/// Filtert die Ergebnisse basierend auf der Bedingung x > 10
/// Gibt die gefilterten Ergebnisse zurück.
pub fn apply_and_filter<Transform, Predicate>(numbers: &[i32], operation: Transform, to_filter: Predicate) -> Option<Vec<i32>>
where
    Transform: Fn(i32) -> i32,
    Predicate: Fn(i32) -> bool,
{
    let filtered: Vec<i32> = numbers
	.iter()
	.copied()  //direkt über Werte iterieren
	.map(|x| operation(x)) // Transformation
	.filter(|&x| to_filter(x)) // Filterung
	.collect();

    if filtered.is_empty() {
        return None;
    }
    Some(filtered)
}

#[derive(Debug)]
pub enum DoubleAndFilterError {
    EmptyInput,
    
}

pub fn double_and_filter<Predicate>(numbers: &[i32], to_filter: Predicate) -> Result<Vec<i32>, DoubleAndFilterError>
where
    Predicate: Fn(i32) -> bool
{
    if numbers.is_empty() {
        return Err(DoubleAndFilterError::EmptyInput);
    }
    
    let filtered:  Vec<i32> = numbers
        .iter()
        .copied()
        .map(|x| &x*2)
        .filter(|&x| to_filter(x))
        .collect();

    Ok(filtered)	
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_and_filter_double() {
        let input = vec![2, 3, 4, 5, 6];
        let output = apply_and_filter(&input, |x| x * 2, |x| x > 10);
        assert_eq!(output, Some(vec![12]));
    }

    #[test]
    fn test_apply_and_filter_no_matches() {
        let input = vec![1, 2, 3, 4];
        let output = apply_and_filter(&input, |x| x - 10, |x| x > 10);
        assert_eq!(output, None)
    }

    #[test]
    fn test_apply_and_filter_with_identity() {
        let input = vec![11, 12, 13];
        let output = apply_and_filter(&input, |x| x, |x| x > 10);
        assert_eq!(output, Some(vec![11, 12, 13]));
    }
}
