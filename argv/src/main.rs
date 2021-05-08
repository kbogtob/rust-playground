use std::{collections::HashMap, env, process::exit};

// get the numbers arguments given to the program
fn get_number_arguments() -> Vec<u32> {
    let args = env::args();

    let mut numbers: Vec<u32> = args
        .map(|arg| arg.trim().parse::<u32>().ok())
        .filter(Option::is_some)
        .map(Option::unwrap)
        .collect();

    numbers.sort();
    numbers
}

// compute the mean of the given number list
fn compute_mean(numbers: &Vec<u32>) -> f64 {
    numbers.iter().fold(0, |sum, number| sum + number) as f64 / numbers.len() as f64
}

// compute the median of the given number list
// it assumed they are sorted
fn compute_median(numbers: &Vec<u32>) -> u32 {
    *numbers.get(numbers.len() / 2).unwrap()
}

// compute the mode of given numbers
fn compute_mode_unsorted(numbers: &Vec<u32>) -> u32 {
    let mut occurences = HashMap::new();

    for number in numbers {
        let count = occurences.entry(number).or_insert(0);

        *count += 1;
    }

    let max_occurence = occurences
        .iter()
        .max_by(|number_pair_a, number_pair_b| number_pair_a.1.cmp(number_pair_b.1))
        .unwrap();
    **max_occurence.0
}

// more optimized in memory and time complexity as the list is sorted
fn compute_mode_sorted(numbers: &Vec<u32>) -> u32 {
    let mut current_count = 0;
    let mut current_number = numbers.first().unwrap();
    let mut max_occurences = 0;
    let mut mode = numbers.first().unwrap();

    for number in numbers {
        if number != current_number {
            // update mode if previous number was more frequent
            if max_occurences < current_count {
                max_occurences = current_count;
                mode = current_number;
            }
            // switch to new number
            current_number = number;
            current_count = 0;
        }
        // increase count of occurences
        current_count += 1;
    }

    *mode
}

fn main() {
    let numbers = get_number_arguments();

    if numbers.is_empty() {
        eprintln!("Usage: stat <number1> <number2> ... <numberN>");
        exit(1);
    }

    let mean = compute_mean(&numbers);
    let median = compute_median(&numbers);
    let mode = compute_mode_sorted(&numbers);

    println!("Sorted numbers = {:?}", numbers);
    println!("mean = {}", mean);
    println!("median = {}", median);
    println!("mode = {}", mode);
}

#[cfg(test)]
mod tests {
    #[test]
    fn compute_mean_works() {
        let numbers = vec![1, 2, 3, 4, 5, 6];
        let mean = super::compute_mean(&numbers);
        assert_eq!(mean, 3.5f64);
    }

    #[test]
    fn compute_mean_works_with_one_element() {
        let numbers = vec![42];
        let mean = super::compute_mean(&numbers);
        assert_eq!(mean, 42f64);
    }

    #[test]
    fn compute_median_works() {
        let numbers = vec![1, 2, 3, 4, 5, 6];
        let median = super::compute_median(&numbers);
        assert_eq!(median, 4);
    }

    #[test]
    fn compute_median_works_with_one_element() {
        let numbers = vec![42];
        let median = super::compute_median(&numbers);
        assert_eq!(median, 42);
    }

    #[test]
    fn compute_mode_unsorted_works() {
        let numbers = vec![1, 2, 3, 4, 4, 5, 6];
        let mode = super::compute_mode_unsorted(&numbers);
        assert_eq!(mode, 4);
    }

    #[test]
    fn compute_mode_sorted_works() {
        let numbers = vec![1, 2, 3, 4, 4, 5, 6];
        let mode = super::compute_mode_sorted(&numbers);
        assert_eq!(mode, 4);
    }
}
