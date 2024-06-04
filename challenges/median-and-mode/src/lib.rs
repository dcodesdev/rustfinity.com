use std::collections::HashMap;

pub fn median(numbers: &mut Vec<i32>) -> f32 {
    numbers.sort();
    let mid = numbers.len() / 2;
    if numbers.len() % 2 == 0 {
        (numbers[mid - 1] as f32 + numbers[mid] as f32) / 2.0
    } else {
        numbers[mid] as f32
    }
}

pub fn mode(numbers: &Vec<i32>) -> Vec<i32> {
    let mut occurrences = HashMap::new();
    for &number in numbers {
        *occurrences.entry(number).or_insert(0) += 1;
    }

    let max_count = occurrences.values().max().unwrap_or(&0);
    let mut modes: Vec<i32> = occurrences
        .iter()
        .filter(|&(_, &count)| count == *max_count)
        .map(|(&number, _)| number)
        .collect();

    modes.sort();
    modes
}
