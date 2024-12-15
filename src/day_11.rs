use std::collections::HashMap;
use cached::proc_macro::cached;

/**
   Reads the stone configuration into a vector
 */
fn preprocess(path: &str) -> Vec<u128> {
    let contents = std::fs::read_to_string(path).expect(path);
    
    let lines: Vec<&str> = contents.lines().collect();
    lines[0].split_whitespace()
            .map(|stone| stone.parse::<u128>().expect("Expected parsable u128"))
            .collect()
}

/**
   Processes a single stone at the given index, and returns the stone(s) that resulted from the processing.
   The resulting vector only resembles the change for the given index, not the entire vector!
   Examples:

   ```rs
   process_stone(0) -> vec![1]
   process_stone(23) -> vec![2, 3]
   ```

   This function uses caching functionality to prevent repeated computation for the same values
 */
#[cached]
fn process_stone(value: u128) -> Vec<u128> {    
    if value == 0 {
        return vec![1];
    }
    
    let value_as_string = value.to_string();
    let length = value_as_string.len();
    // We can be sure, that the len represents the actual number of digits,
    // since it was created from a u128, which means all chars are ascii.
    // If the number of digits is even, split them in two.
    if length % 2 == 0 {
        let half = length / 2;
        let slice = value_as_string.as_str();
        let first: u128 = slice[..half].parse().expect("Expected a parsable u128");
        let second: u128 = slice[half..].parse().expect("Expected a parsable u128");
        return vec![first, second];
    }

    vec![value * 2024]
}

/**
   Steps through each stone in the stones count map and applies the rules. The result is the new
   stones count map after a full iteration
 */
fn iterate(stones_count_map: &HashMap<u128, u128>) -> HashMap<u128, u128> {
    let mut new_stones_count_map: HashMap<u128, u128> = HashMap::new();
    
    // For each unique stone engraving in the map, compute the resulting stone(s)
    // Then insert the found stones into the result map with the original count.
    // If there are five stones with engraving 2 in the original map, the same
    // stone result will be contained five times after one iteration as well.
    for (stone, count) in stones_count_map.into_iter() {
        let new_stones = process_stone(*stone);
        for new_stone in &new_stones {
            if let Some(c) = new_stones_count_map.get(new_stone) {
                // In case the same result was already contained in the result map,
                // both counts are just summed up during the update.
                new_stones_count_map.insert(*new_stone, *c + *count);
            } else {
                new_stones_count_map.insert(*new_stone, *count);
            }
        }
    }

    new_stones_count_map
}

/**
   Returns the number of stones after the given number of iterations
 */
pub fn both_parts(path: &str, iterations: u8) -> u128 {
    let stones = preprocess(path);
    // Stores the number of occurences for each stone value
    let mut stones_count_map: HashMap<u128, u128> = HashMap::new();

    for stone in &stones {
        if let Some(count) = stones_count_map.get(stone) {
            stones_count_map.insert(*stone, (*count) + 1);
        } else {
            stones_count_map.insert(*stone, 1);
        }
    }

    for _ in 0..iterations {
        stones_count_map = iterate(&stones_count_map);
    }

    let amount = stones_count_map.values().sum();
    println!("After {} iterations there are {} stones", iterations, amount);
    amount
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_preprocess() {
        let stones = preprocess("./inputs/day_11.txt");

        assert_eq!(stones, vec![7725, 185, 2, 132869, 0, 1840437, 62, 26310]);
    }

    #[test]
    fn test_process_stone() {
        assert_eq!(process_stone(0), vec![1]);
        assert_eq!(process_stone(23), vec![2, 3]);
        assert_eq!(process_stone(2), vec![4048]);
        assert_eq!(process_stone(4001), vec![40, 1]);
    }

    #[test]
    fn test_part_1() {
        let result = both_parts("./inputs/day_11.txt", 1);
        assert_eq!(result, 11);
    }

    #[test]
    fn test_iterate() {
        let mut map: HashMap<u128, u128> = HashMap::new();
        map.insert(0, 1);
        map.insert(23, 1);
        map.insert(99, 1);
        map.insert(2, 1);
        let result = iterate(&map);
        let mut keys: Vec<u128> = result.keys().map(|k| *k).collect();
        keys.sort();

        assert_eq!(keys, vec![1, 2, 3, 9, 4048]);
        assert_eq!(*result.get(&9).unwrap(), 2);
    }
}