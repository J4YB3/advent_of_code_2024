/**
 * Returns a vector of lines. Each line itself is a vector of u8 integers.
 */
fn preprocess(path: &str) -> Vec<Vec<u8>> {
    let contents = std::fs::read_to_string(path).expect(path);
    let lines_iterator = contents.lines().map(|x| x.to_string());

    let lines: Vec<Vec<u8>> = lines_iterator
        .map(|line| line.split_whitespace()
            .map(|v| v.parse().expect("Number should be u8!"))
            .collect())
        .collect();

    lines
}

/**
   Verifies that the two line rules apply to the given line:
   - Each element in the line increases or decreases
   - The difference between two consecutive elements must be less or equal to 3
 */
fn line_is_safe(line: Vec<u8>) -> bool {
    let mut safe = true;
    let is_increasing = line[0] < line[1];

    for i in 0..(line.len() - 1) {
        safe = safe && line[i].abs_diff(line[i+1]) <= 3;
        safe = safe && if is_increasing { line[i] < line[i+1] } else { line[i] > line[i+1] };
    }

    safe
}

pub fn part_1(path: &str) -> u16 {
    let lines = preprocess(path);
    let counter: u16 = lines.iter()
        .map(|line| if line_is_safe(line.to_vec()) { 1 } else { 0 })
        .sum();

    println!("Safe lines: {}", counter);
    counter
}

/**
   Brute force approach by removing one element at a time and
   then checking if the line would be safe without it.
 */
fn line_is_safe_with_dampener(line: Vec<u8>) -> bool {
    let line_is_already_safe = line_is_safe(line.clone());

    if line_is_already_safe {
        return true;
    }

    for i in 0..line.len() {
        let mut line_copy = line.clone();
        line_copy.remove(i);
        if line_is_safe(line_copy) {
            return true;
        };
    }

    false
}

pub fn part_2(path: &str) -> u16 {
    let lines = preprocess(path);
    let counter: u16 = lines.iter()
        .map(|line| if line_is_safe_with_dampener(line.to_vec()) { 1 } else { 0 })
        .sum();

    println!("Safe lines: {}", counter);
    counter
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_line_is_safe() {
        assert!(line_is_safe(vec![1, 2, 3, 4, 5]));
        assert!(line_is_safe(vec![9, 8, 7, 6, 5]));
        assert!(line_is_safe(vec![1, 2]));
        assert!(line_is_safe(vec![9, 8, 7]));
        assert!(line_is_safe(vec![1, 4, 7, 10]));
    }

    #[test]
    fn test_line_is_unsafe() {
        assert!(!line_is_safe(vec![1, 5, 6, 7, 8]));
        assert!(!line_is_safe(vec![9, 5, 4, 3, 2]));
    }

    #[test]
    fn test_part_1() {
        let result = part_1("./inputs/day_02_test.txt");
        assert_eq!(result, 2);
    }

    #[test]
    fn test_line_is_safe_with_dampener() {
        assert!(line_is_safe_with_dampener(vec![1, 9, 3, 4, 5]));
        assert!(line_is_safe_with_dampener(vec![9, 1, 7, 6, 5]));
        assert!(line_is_safe_with_dampener(vec![1, 2, 3, 4, 5]));
        assert!(line_is_safe_with_dampener(vec![1, 3, 2, 4, 5]));
        assert!(line_is_safe_with_dampener(vec![8, 6, 4, 4, 1]));
    }

    #[test]
    fn test_line_is_unsafe_with_dampener() {
        assert!(!line_is_safe_with_dampener(vec![1, 9, 8, 4, 5]));
        assert!(!line_is_safe_with_dampener(vec![9, 1, 2, 6, 5]));
        assert!(!line_is_safe_with_dampener(vec![1, 2, 7, 8, 9]));
    }

    #[test]
    fn test_part_2() {
        let result = part_2("./inputs/day_02_test.txt");
        assert_eq!(result, 4);
    }
}