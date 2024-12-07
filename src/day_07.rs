use std::time::Instant;

#[derive(Debug, Copy, Clone, PartialEq)]
enum Operator {
    Add,
    Mul,
    Concat,
}

const OPERATORS: [Operator; 2] = [Operator::Add, Operator::Mul];
const OPERATORS_PART_2: [Operator; 3] = [Operator::Add, Operator::Mul, Operator::Concat];

impl Operator {
    /**
     * Generates all possible combinations of the enum values for the given length recursively
     */
    fn generate_combinations(length: usize, include_concat: bool) -> Vec<Vec<Self>> {
        if length == 0 {
            return vec![vec![]];
        }

        // Get the combinations for the smaller length
        let shorter_combinations = Self::generate_combinations(length - 1, include_concat);
        let mut combinations: Vec<Vec<Operator>> = vec![];
        let ops_to_use = if include_concat { OPERATORS_PART_2.to_vec() } else { OPERATORS.to_vec() };

        for combination in shorter_combinations {
            for op in &ops_to_use {
                let mut combination_clone = combination.clone();
                combination_clone.push(*op);
                combinations.push(combination_clone);
            }
        }

        combinations
    }
}

struct Equation {
    expected_result: u64,
    numbers: Vec<u64>,
}

impl Equation {
    /**
       Brute-forces a result by computing all operator combinations of the required
       length and testing them until one is found that makes the equation true.
     */
    pub fn valid(&self, all_combinations: &Vec<Vec<Vec<Operator>>>) -> bool {
        let combinations = &all_combinations[self.numbers.len()];

        for operator_combination in combinations {
            let result_option = self.evaluate(operator_combination.clone());
            // Something is wrong with the operator combinations
            if result_option.is_none() {
                return false;
            }

            // We found a combination of operators where the equation becomes valid
            if result_option.unwrap() == self.expected_result {
                return true;
            }
        }

        false
    }

    /**
       Takes a vector of operators which length has to be exactly one less than
       the numbers of the equation. Then evaluates the equation with the given
       operators and returns the result. Returns None if the stated condition fails.
     */
    pub fn evaluate(&self, operators: Vec<Operator>) -> Option<u64> {
        if operators.len() != self.numbers.len() - 1 {
            return None;
        }

        let mut result: u64 = self.numbers[0];

        for i in 1..self.numbers.len() {
            result = match operators[i-1] {
                Operator::Add => result + self.numbers[i],
                Operator::Mul => result * self.numbers[i],
                Operator::Concat => format!("{}{}", result, self.numbers[i]).parse().expect("Should be a parsable u64"),
            };

            if result > self.expected_result {
                return Some(0);
            }
        }
        
        Some(result)
    }
}

/**
   Creates the vector of equations for the file content in path.
 */
fn preprocess(path: &str) -> (Vec<Equation>, usize) {
    let contents = std::fs::read_to_string(path).expect(path);
    let equation_lines: Vec<Vec<String>> = contents.lines().map(|x| x.to_string().split_whitespace().map(|y| y.to_string()).collect()).collect();
    let mut longest_line_length: usize = 0;

    let mut equations = vec![];
    for line in equation_lines {
        let mut expected_result: u64 = 0;
        let mut numbers: Vec<u64> = vec![];
        for i in 0..line.len() {
            if i > 0 {
                numbers.push(line[i].parse().expect("Should be a parsable u64"));
                continue;
            }

            expected_result = line[0].strip_suffix(":").unwrap().parse().expect("Should be a parsable u64");
        }

        if numbers.len() > longest_line_length {
            longest_line_length = numbers.len();
        }

        equations.push(Equation { expected_result, numbers });
    }

    (equations, longest_line_length)
}

pub fn part_1(path: &str) -> u64 {
    let (equations, longest_line_length) = preprocess(path);
    // Holds all possible combinations for all occurring line lengths, indexed by their line length
    // Initialize this by the empty combinations for line length 0 to keep the indexing correct
    let mut combinations: Vec<Vec<Vec<Operator>>> = vec![vec![vec![]]];

    for length in 1..=longest_line_length {
        combinations.push(Operator::generate_combinations(length - 1, false));
    }

    let mut sum: u64 = 0;

    for equation in equations {
        sum += if equation.valid(&combinations) { equation.expected_result } else { 0 };
    }

    println!("The sum of the valid equations is {}", sum);
    sum
}

pub fn part_2(path: &str) -> u64 {
    let (equations, longest_line_length) = preprocess(path);
    // Holds all possible combinations for all occurring line lengths, indexed by their line length
    // Initialize this by the empty combinations for line length 0 to keep the indexing correct
    let mut combinations: Vec<Vec<Vec<Operator>>> = vec![vec![]];

    for length in 1..=longest_line_length {
        combinations.push(Operator::generate_combinations(length - 1, true));
    }

    let mut sum: u64 = 0;

    for equation in equations {
        sum += if equation.valid(&combinations) { equation.expected_result } else { 0 };
    }

    println!("The sum of the valid equations is {}", sum);
    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_preprocess() {
        let (equations, longest_line_length) = preprocess("./inputs/day_07_test.txt");
        assert_eq!(equations.len(), 9);
        assert_eq!(longest_line_length, 4);
        assert_eq!(equations[0].expected_result, 190);
        assert_eq!(equations[0].numbers, vec![10, 19]);
    }

    #[test]
    fn test_equation_valid_for_part_1() {
        let combinations = vec![vec![], vec![], vec![vec![Operator::Add], vec![Operator::Mul]]];

        let eq = Equation { expected_result: 100, numbers: vec![10, 10] };
        assert!(eq.valid(&combinations));

        let eq = Equation { expected_result: 50, numbers: vec![10, 10] };
        assert!(!eq.valid(&combinations));

        let eq = Equation { expected_result: 20, numbers: vec![10, 10] };
        assert!(eq.valid(&combinations));
    }
    
    #[test]
    fn test_equation_evaluate() {
        let eq = Equation { expected_result: 100, numbers: vec![10, 10] };
        assert_eq!(eq.evaluate(vec![Operator::Add]), Some(20));
        assert_eq!(eq.evaluate(vec![Operator::Mul]), Some(100));
        assert_eq!(eq.evaluate(vec![]), None);
        assert_eq!(eq.evaluate(vec![Operator::Add, Operator::Mul]), None);

        let eq = Equation { expected_result: 100, numbers: vec![25, 25, 2] };
        assert_eq!(eq.evaluate(vec![Operator::Add, Operator::Mul]), Some(100));
    }

    #[test]
    fn test_operator_generate_combinations() {
        let combinations = Operator::generate_combinations(2, false);
        assert_eq!(combinations, vec![
            vec![Operator::Add, Operator::Add],
            vec![Operator::Add, Operator::Mul],
            vec![Operator::Mul, Operator::Add],
            vec![Operator::Mul, Operator::Mul],
        ]);

        let combinations = Operator::generate_combinations(2, true);
        assert_eq!(combinations, vec![
            vec![Operator::Add, Operator::Add],
            vec![Operator::Add, Operator::Mul],
            vec![Operator::Add, Operator::Concat],
            vec![Operator::Mul, Operator::Add],
            vec![Operator::Mul, Operator::Mul],
            vec![Operator::Mul, Operator::Concat],
            vec![Operator::Concat, Operator::Add],
            vec![Operator::Concat, Operator::Mul],
            vec![Operator::Concat, Operator::Concat],
        ]);
    }

    #[test]
    fn test_part_1() {
        let result = part_1("./inputs/day_07_test.txt");
        assert_eq!(result, 3749);
    }

    #[test]
    fn test_part_2() {
        let result = part_2("./inputs/day_07_test.txt");
        assert_eq!(result, 11387);
    }
}