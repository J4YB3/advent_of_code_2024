use regex::Regex;

/**
   Returns the line containing the instructions from the input file
 */
fn preprocess(path: &str) -> String {
    std::fs::read_to_string(path).expect(path)
}

pub fn part_1(path: &str) -> u32 {
    let input = preprocess(path);
    let expression = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut sum: u32 = 0;
    for (_, [number_1, number_2]) in expression.captures_iter(input.as_str()).map(|c| c.extract()) {
        sum += number_1.parse::<u32>().expect("Should be a parsable u32")
            * number_2.parse::<u32>().expect("Should be a parsable u32");
    }

    println!("Result: {}", sum);
    sum
}

pub fn part_2(path: &str) -> u32 {
    let input = preprocess(path);
    let expression = Regex::new(r"mul\((\d{1,3},\d{1,3})\)|(do)\(\)|(don't)\(\)").unwrap();

    let mut sum: u32 = 0;
    let mut enabled = true;
    for (_, [capture]) in expression.captures_iter(input.as_str()).map(|c| c.extract()) {
        match capture {
            "do" => enabled = true,
            "don't" => enabled = false,
            numbers => {
                if !enabled {
                    continue;
                }

                // Calculate the product of the two numbers
                sum += numbers.split(',')
                    .fold(
                        1,
                        |acc: u32, n| acc * n.parse::<u32>().expect("Should be a parsable u32"),
                    );
            }
        }
    }

    println!("Result: {}", sum);
    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("./inputs/day_03_test_part1.txt"), 161);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2("./inputs/day_03_test_part2.txt"), 48);
    }
}