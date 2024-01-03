use colored::Colorize;
use num_bigint::BigUint;
use num_traits::{One, Zero};
use std::io;

fn main() {
    let nth = prompt_for_nth();
    let print_sequence = prompt_to_print_sequence();

    let sequence = fibonacci(nth);

    println!(
        "\n➡️ The value in the position {} is: {}",
        nth,
        get_fibonacci_number_at_position(nth)
    );

    if print_sequence {
        println!("\n➡️ {:?}", sequence);
    }
}

fn prompt_for_nth() -> usize {
    let mut nth = String::new();

    loop {
        println!("{}", "\nEnter the position:".bright_blue());
        nth.clear();
        io::stdin()
            .read_line(&mut nth)
            .expect("Failed to read line");

        match nth.trim().parse::<usize>() {
            Ok(v) => return v,
            Err(_) => {
                println!(
                    "{}",
                    "Invalid input. Please enter a valid position.".yellow()
                );
                continue;
            }
        };
    }
}

fn prompt_to_print_sequence() -> bool {
    let mut print_sequence = String::new();

    loop {
        println!(
            "{}",
            "\nDo you want to print the entire sequence? (y/n)".bright_blue()
        );
        print_sequence.clear();
        io::stdin()
            .read_line(&mut print_sequence)
            .expect("Failed to read line");

        match print_sequence.trim().to_lowercase().as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => {
                println!("{}", "Invalid input. Please enter 'yes' or 'no'.".yellow());
                continue;
            }
        }
    }
}

fn fibonacci(nth: usize) -> Vec<BigUint> {
    let mut sequence = Vec::<BigUint>::new();
    let mut n1: BigUint = Zero::zero();
    let mut n2: BigUint = One::one();

    for _ in 0..nth {
        let sum = n1 + &n2;
        sequence.push(sum.clone());
        n1 = n2;
        n2 = sum;
    }

    return sequence;
}

fn get_fibonacci_number_at_position(nth: usize) -> BigUint {
    let sequence = fibonacci(nth);
    sequence[nth - 1].clone()
}

#[cfg(test)]
mod tests {
    use crate::{fibonacci, get_fibonacci_number_at_position};
    use num_bigint::BigUint;

    #[test]
    fn test_fibonacci() {
        let sequence = fibonacci(10);
        assert_eq!(
            sequence,
            vec![
                BigUint::from(1u32),
                BigUint::from(2u32),
                BigUint::from(3u32),
                BigUint::from(5u32),
                BigUint::from(8u32),
                BigUint::from(13u32),
                BigUint::from(21u32),
                BigUint::from(34u32),
                BigUint::from(55u32),
                BigUint::from(89u32),
            ]
        );
    }

    #[test]
    fn test_get_fibonacci_number_at_position() {
        assert_eq!(get_fibonacci_number_at_position(1), BigUint::from(1u32));
        assert_eq!(get_fibonacci_number_at_position(2), BigUint::from(2u32));
        assert_eq!(get_fibonacci_number_at_position(3), BigUint::from(3u32));
        assert_eq!(get_fibonacci_number_at_position(4), BigUint::from(5u32));
        assert_eq!(get_fibonacci_number_at_position(5), BigUint::from(8u32));
        assert_eq!(get_fibonacci_number_at_position(6), BigUint::from(13u32));
        assert_eq!(get_fibonacci_number_at_position(7), BigUint::from(21u32));
        assert_eq!(get_fibonacci_number_at_position(8), BigUint::from(34u32));
        assert_eq!(get_fibonacci_number_at_position(9), BigUint::from(55u32));
        assert_eq!(get_fibonacci_number_at_position(10), BigUint::from(89u32));
    }
}
