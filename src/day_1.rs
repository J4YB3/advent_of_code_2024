use std::fs;
use core::slice::Iter;

/**
 * Returns the pre-processed vectors of the two input columns.
 * The vectors are already sorted from smallest to largest numbers
 */
fn preprocess() -> (Vec<u32>, Vec<u32>) {
    let path = "inputs/day_1.txt";
    let contents = fs::read_to_string(path).expect(path);
    let lines_iterator = contents.lines().map(|x| x.to_string());

    let mut list_1: Vec<u32> = vec![];
    let mut list_2: Vec<u32> = vec![];

    lines_iterator
        .for_each(|line| {
            let mut line_iterator = line.split_whitespace();
            list_1.push(line_iterator.next().unwrap().parse().unwrap());
            list_2.push(line_iterator.next().unwrap().parse().unwrap());
        });

    list_1.sort();
    list_2.sort();

    (list_1, list_2)
}

/**
   Sums up the differences between the left and right list rows
 */
pub fn part_1() {
    let (list_1, list_2) = preprocess();

    let result: u32 = list_1.iter().zip(list_2.iter())
        .map(|(left, right)| left.abs_diff(*right))
        .sum();

    println!("Result: {}", result);
}

/**
   Counts the occurrences of the given number *of* and returns the number of
   occurences and the next number.
   Expects an iterator over a sorted list. Modifies the given iterator!
 */
fn count_occurences(iter: &mut Iter<'_, u32>, of: u32) -> (u32, Option<u32>) {
    let mut count: u32 = 0;
    let mut next_number: u32;

    loop {
        let next_number_option = iter.next();
        if next_number_option.is_none() {
            return (count, None);
        }

        next_number = *next_number_option.unwrap();
        
        if next_number != of {
            break;
        }

        count += 1;
    }

    (count, Some(next_number))
}

/**
   Implements a dual pointer approach for the second part of the first day.
 */
pub fn part_2() {
    let (list_1, list_2) = preprocess();

    let mut similarity_score: u32 = 0;
    let mut iter_1 = list_1.iter();
    let mut iter_2 = list_2.iter();

    let mut current_right_number = 0;
    let mut next_right_number_option: Option<u32> = Some(*iter_2.next().unwrap());
    let mut current_right_sum: u32 = 0;

    'outer: while let Some(current_left_number) = iter_1.next() {
        // If the number is smaller than the next summed up number, we can skip it
        if *current_left_number < current_right_number {
            continue;
        }
        
        // If the current number is greater than the current summed up number,
        // evaluate the new count before the similarity score is calculated.
        // Skip right numbers until one is found that is greater than the current left.
        while *current_left_number > current_right_number {
            // We can break the loop early if there are no more numbers in the right list.
            // Because then all remaining left numbers will have 0 occurences in the right list.
            if next_right_number_option.is_none() {
                break 'outer;
            }

            current_right_number = next_right_number_option.unwrap();
            (current_right_sum, next_right_number_option) = count_occurences(&mut iter_2, current_right_number);
            current_right_sum += 1;
        }
        
        // Similarity score should increase by the occurrences in right list times the current number,
        // only if the numbers are equal. This might not be the case, because a new right number might
        // have been picked inside the while loop above
        if *current_left_number == current_right_number {
            similarity_score += current_left_number * current_right_sum;
        }
    }

    println!("Result: {}", similarity_score);
}