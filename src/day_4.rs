/**
   Returns a vector of lines, where each line is itself a vector of characters.
 */
fn preprocess(path: &str) -> Vec<Vec<char>> {
    let contents = std::fs::read_to_string(path).expect(path);
    let lines_iterator = contents.lines().map(|x| x.to_string());

    let lines: Vec<Vec<char>> = lines_iterator
        .map(|line| line.chars().collect())
        .collect();

    lines
}

/**
   Beginning from (x, y), step through the chars in all eight directions and check if the word
   "XMAS" can be formed. Returns the number of found matches.
 */
fn count_xmas_apperances_from(chars: &Vec<Vec<char>>, x: usize, y: usize) -> u32 {
    if chars[y][x] != 'X' {
        return 0;
    }

    // These are the directions that we are going to check
    let directions: Vec<(i32, i32)> = vec![(1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1), (0, -1), (1, -1)];
    // We already checked that we are starting with an 'X', so we only need to check the remaining characters in this order
    let expected_characters = vec!['M', 'A', 'S'];
    // The maximum bounds of the vector
    let max_x: i32 = (chars[0].len() as i32) - 1;
    let max_y: i32 = (chars.len() as i32) - 1;
    let mut sum: u32 = 0;

    'outer: for i in 0..directions.len() {
        // Set up the local variables for each direction iteration
        let direction = &directions[i];
        let mut current_offset = direction.clone();

        for expected_character in &expected_characters {
            // Find out if we are still in bounds of the vector
            let current_x: i32 = (x as i32) + current_offset.0;
            let current_y: i32 = (y as i32) + current_offset.1;
            if current_x > max_x || current_x < 0 || current_y > max_y || current_y < 0 {
                // We don't want to use break here, because then the sum would go up
                continue 'outer;
            }

            // We know now, that we're in bounds, so we can parse the i32 as usize to index the vector
            let x_index = current_x as usize;
            let y_index = current_y as usize;
            if chars[y_index][x_index] != *expected_character {
                // We don't want to use break here, because then the sum would go up
                continue 'outer;
            }

            // Go one step further into the currently selected direction
            current_offset = (current_offset.0 + direction.0, current_offset.1 + direction.1);
        }

        // Add one appearance when the inner for loop finishes normally, because then all checks worked
        sum += 1;
    }

    sum
}

pub fn part_1(path: &str) -> u32 {
    let chars = preprocess(path);
    let mut sum: u32 = 0;

    for y in 0..chars.len() {
        let line = &chars[y];

        for x in 0..line.len() {
            sum += count_xmas_apperances_from(&chars, x, y);
        }
    }

    println!("XMAS appeared {} times", sum);
    sum
}

/**
   The slightly different spelling for xmas and x_mas are deliberate, since the task specifically mentioned it differently (XMAS -> X-MAS).
   Beginning from (x, y), check if the given position results in an X-MAS appearance. An X-MAS appearance are the word MAS in an X-shape.
   Returns true in case this pattern is detected, and false otherwise.
   
   # Examples
   ```
   M . M  |  S . M  |  S . S  |  M . S
   . A .  |  . A .  |  . A .  |  . A .
   S . S  |  S . M  |  M . M  |  M . S
   ```

   Upon inspection we notice a pattern. The letters M and S always need to be located in the same row or column and can never
   appear in the same diagonal together. Additionally, the letter A always needs to be in the center. The function uses these
   facts to speed up the processing slightly.
 */
fn is_x_mas_appearance_from(chars: &Vec<Vec<char>>, x: usize, y: usize) -> bool {
    let max_x = chars[0].len() - 1;
    let max_y = chars.len() - 1;
    // The given (x, y) position is on the edge of the word puzzle, so it already can't be a X-MAS appearance
    let out_of_bounds = x < 1 || x > max_x - 1 || y < 1 || y > max_y - 1;

    if chars[y][x] != 'A' || out_of_bounds {
        return false;
    }

    // Define the top-left-bottom-right diagonal chars, and the same for the other diagonal
    let tlbr_chars = vec![chars[y-1][x-1], chars[y+1][x+1]];
    let trbl_chars = vec![chars[y-1][x+1], chars[y+1][x-1]];
    let test_vector_forward = vec!['M', 'S'];
    let test_vector_backward = vec!['S', 'M'];
    
    // Now test that the two diagonals each form the word MAS, forwards or backwards
    if (tlbr_chars == test_vector_forward || tlbr_chars == test_vector_backward)
            && (trbl_chars == test_vector_forward || trbl_chars == test_vector_backward)  {
        return true;
    }

    return false;
}

pub fn part_2(path: &str) -> u32 {
    let chars = preprocess(path);
    let mut sum: u32 = 0;

    for y in 0..chars.len() {
        let line = &chars[y];

        for x in 0..line.len() {
            sum += if is_x_mas_appearance_from(&chars, x, y) { 1 } else { 0 };
        }
    }

    println!("X-MAS appeared {} times", sum);
    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_horizontal_simple() {
        let chars = vec![
            vec!['X', 'M', 'A', 'S']
        ];

        assert_eq!(count_xmas_apperances_from(&chars, 0, 0), 1);
        assert_eq!(count_xmas_apperances_from(&chars, 1, 0), 0);
        assert_eq!(count_xmas_apperances_from(&chars, 2, 0), 0);
        assert_eq!(count_xmas_apperances_from(&chars, 3, 0), 0);
    }

    #[test]
    fn test_count_horizontal_double() {
        let chars = vec![
            vec!['S', 'A', 'M', 'X', 'M', 'A', 'S']
        ];

        assert_eq!(count_xmas_apperances_from(&chars, 0, 0), 0);
        assert_eq!(count_xmas_apperances_from(&chars, 1, 0), 0);
        assert_eq!(count_xmas_apperances_from(&chars, 2, 0), 0);
        assert_eq!(count_xmas_apperances_from(&chars, 3, 0), 2);
        assert_eq!(count_xmas_apperances_from(&chars, 4, 0), 0);
        assert_eq!(count_xmas_apperances_from(&chars, 5, 0), 0);
        assert_eq!(count_xmas_apperances_from(&chars, 6, 0), 0);
    }

    #[test]
    fn test_count_vertical_simple() {
        let chars = vec![
            vec!['X'],
            vec!['M'],
            vec!['A'],
            vec!['S'],
        ];

        assert_eq!(count_xmas_apperances_from(&chars, 0, 0), 1);
        assert_eq!(count_xmas_apperances_from(&chars, 0, 1), 0);
        assert_eq!(count_xmas_apperances_from(&chars, 0, 2), 0);
        assert_eq!(count_xmas_apperances_from(&chars, 0, 3), 0);
    }

    #[test]
    fn test_count_vertical_double() {
        let chars = vec![
            vec!['S'],
            vec!['A'],
            vec!['M'],
            vec!['X'],
            vec!['M'],
            vec!['A'],
            vec!['S'],
        ];

        assert_eq!(count_xmas_apperances_from(&chars, 0, 0), 0);
        assert_eq!(count_xmas_apperances_from(&chars, 0, 1), 0);
        assert_eq!(count_xmas_apperances_from(&chars, 0, 2), 0);
        assert_eq!(count_xmas_apperances_from(&chars, 0, 3), 2);
        assert_eq!(count_xmas_apperances_from(&chars, 0, 4), 0);
        assert_eq!(count_xmas_apperances_from(&chars, 0, 5), 0);
        assert_eq!(count_xmas_apperances_from(&chars, 0, 6), 0);
    }

    #[test]
    fn test_count_diagonal_simple() {
        let chars = vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'M', 'X', 'X'],
            vec!['X', 'X', 'A', 'X'],
            vec!['X', 'X', 'X', 'S'],
        ];

        for y in 0..chars.len() {
            let line = &chars[y];
            for x in 0..line.len() {
                if x == 0 && y == 0 {
                    assert_eq!(count_xmas_apperances_from(&chars, x, y), 1);
                    continue;
                }

                assert_eq!(count_xmas_apperances_from(&chars, x, y), 0);
            }
        }
    }

    #[test]
    fn test_count_all() {
        let chars = vec![
            vec!['S', '.', '.', 'S', '.', '.', 'S'],
            vec!['.', 'A', '.', 'A', '.', 'A', '.'],
            vec!['.', '.', 'M', 'M', 'M', '.', '.'],
            vec!['S', 'A', 'M', 'X', 'M', 'A', 'S'],
            vec!['.', '.', 'M', 'M', 'M', '.', '.'],
            vec!['.', 'A', '.', 'A', '.', 'A', '.'],
            vec!['S', '.', '.', 'S', '.', '.', 'S'],
        ];
        
        for y in 0..chars.len() {
            let line = &chars[y];
            for x in 0..line.len() {
                if x == 3 && y == 3 {
                    assert_eq!(count_xmas_apperances_from(&chars, x, y), 8);
                    continue;
                }

                assert_eq!(count_xmas_apperances_from(&chars, x, y), 0);
            }
        }
    }

    #[test]
    fn test_count_corner() {
        let chars = vec![
            vec!['X', 'M', 'A', 'S'],
            vec!['M', 'M', 'X', 'X'],
            vec!['A', 'X', 'A', 'X'],
            vec!['S', 'X', 'X', 'S'],
        ];

        for y in 0..chars.len() {
            let line = &chars[y];
            for x in 0..line.len() {
                if x == 0 && y == 0 {
                    assert_eq!(count_xmas_apperances_from(&chars, x, y), 3);
                    continue;
                }

                assert_eq!(count_xmas_apperances_from(&chars, x, y), 0);
            }
        }
    }

    #[test]
    fn test_count_none() {
        let chars = vec![
            vec!['.', '.', '.', '.'],
            vec!['.', '.', '.', '.'],
            vec!['.', '.', '.', '.'],
            vec!['.', '.', '.', '.'],
        ];

        for y in 0..chars.len() {
            let line = &chars[y];
            for x in 0..line.len() {
                assert_eq!(count_xmas_apperances_from(&chars, x, y), 0);
            }
        }
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("./inputs/day_04_test.txt"), 18);
    }

    #[test]
    fn test_x_mas_simple() {
        let assertions = |chars: &Vec<Vec<char>>| {
            for y in 0..chars.len() {
                let line = &chars[y];
                for x in 0..line.len() {
                    if x == 1 && y == 1 {
                        assert!(is_x_mas_appearance_from(chars, x, y));
                        continue;
                    }

                    assert!(!is_x_mas_appearance_from(chars, x, y));
                }
            }
        };

        let chars = vec![
            vec!['M', '.', 'S'],
            vec!['.', 'A', '.'],
            vec!['M', '.', 'S'],
        ];
        assertions(&chars);

        let chars = vec![
            vec!['M', '.', 'M'],
            vec!['.', 'A', '.'],
            vec!['S', '.', 'S'],
        ];
        assertions(&chars);

        let chars = vec![
            vec!['S', '.', 'M'],
            vec!['.', 'A', '.'],
            vec!['S', '.', 'M'],
        ];
        assertions(&chars);

        let chars = vec![
            vec!['S', '.', 'S'],
            vec!['.', 'A', '.'],
            vec!['M', '.', 'M'],
        ];
        assertions(&chars);
    }

    #[test]
    fn test_x_mas_interleaved() {
        let chars = vec![
            vec!['S', 'M', 'S', 'S'],
            vec!['.', 'A', 'A', '.'],
            vec!['M', 'M', 'M', 'S'],
        ];

        for y in 0..chars.len() {
            let line = &chars[y];
            for x in 0..line.len() {
                if (x == 1 && y == 1) || (x == 2 && y == 1) {
                    assert!(is_x_mas_appearance_from(&chars, x, y));
                    continue;
                }

                assert!(!is_x_mas_appearance_from(&chars, x, y));
            }
        }
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2("./inputs/day_04_test.txt"), 9);
    }
}