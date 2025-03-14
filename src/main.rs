use day01::{filter_odd, squares, sum};
use day02::{filter_by_divisor, transform_and_filter};
use day03::{apply_and_filter, double_and_filter};
use day04::RingBuffer;
use day05::factorial_tail_recursion;
use day06::{process_numbers, process_with_debug};
use day07::process_and_sum;
use std::time::Instant;

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

    let mut buffer = RingBuffer::new(3);
    buffer.push(10);
    buffer.push(20);
    buffer.push(30);
    buffer.push(40);

    while let Some(item) = buffer.pop() {
        println!("Ringbuffer-Element: {}", item);
    }

    let ftr = factorial_tail_recursion(10.0, 1.0);
    println!("Ergebnis der Fakultät von 10: {}", ftr);

    let ftr = factorial_tail_recursion(1000.0, 1.0);
    println!("Ergebnis der Fakultät von 1000: {}", ftr);

    let sum = process_numbers(&numbers);
    println!(
        "Das Ergebnis der Zahlen, die durch 3 teilbar sind, mal 2: {}",
        sum
    );

    let sum = process_with_debug(&numbers);
    println!(
        "Das Ergebnis der Zahlen, die durch 3 teilbar sind, mal 2: {}",
        sum
    );

    let large_data: Vec<i32> = (1..50_001).collect();

    let start = Instant::now();
    let result_iter = process_numbers(&large_data);
    let duration_iter = start.elapsed();
    println!(
        "Iterator Result: {}, Time: {:?}",
        result_iter, duration_iter
    );

    let start = Instant::now();
    let result_direct = process_numbers(&large_data);
    let duration_direct = start.elapsed();
    println!(
        "Direct Result: {}, Time: {:?}",
        result_direct, duration_direct
    );

    let sum = process_and_sum(&numbers);
    println!(
        "Summe aller positiven Zahlen, die verdoppelt wurden und durch 5 teilbar sind: {}",
        sum
    );
}
