use day01::{filter_odd, squares, sum};
use day02::{filter_by_divisor, transform_and_filter};
use day03::{apply_and_filter, double_and_filter};

fn main() {
    println!("Rust Functional Programming Tutorial - Tag 01");

    let numbers = vec![1, 2, 3, 4, 5];
    let divisor = 2;

    let squares_result = squares(&numbers);
    println!("Squares: {:?}", squares_result);

    let odd_numbers = filter_odd(&numbers);
    println!("Odd numbers: {:?}", odd_numbers);

    let sum = sum(&numbers);
    println!("Sum of number: {}", sum);

    if let Some(filtered) = filter_by_divisor(&numbers, divisor) {
        println!("Gefillterte Zahlen: {:?}", filtered);
    }

    if let Some(filtered) = transform_and_filter(&numbers, divisor) {
        println!("Tranformiert und gefilterte Zahlen: {:?}", filtered);
    } else {
        println!("Es wurden keine Zahlen gefunden zum Transformieren und Filtern");
    }

    if let Some(filtered) = apply_and_filter(&numbers, |x| x * 3, |x| x > 2) {
        println!("Funktion ausgeführt und gefilterte Zahlen:  {:?}", filtered);
    } else {
        println!("Es wurden keine Zahlen gefunden, die größer 10 sind.")
    }

    if let Ok(filtered) = double_and_filter(&numbers, |x| x % 3 == 0) {
	println!("Funktion ausgeführt und gefilterte Zahlen: {:?}", filtered);
    } else {
	println!("Es wurden keine Zahlen gefunden, die verdoppelt und anschließend durch 3 teilbar sind.")
    }
}
