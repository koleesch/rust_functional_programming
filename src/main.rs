use day01::{filter_odd, squares};

fn main() {
    println!("Rust Functional Programming Tutorial - Tag 01");

    let numbers = vec![1, 2, 3, 4, 5];
    let squares_result = squares(numbers.clone());
    println!("Squares: {:?}", squares_result);

    let odd_numbers = filter_odd(numbers);
    println!("Odd numbers: {:?}", odd_numbers);
}
