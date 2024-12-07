const UP: (isize, isize) = (0, -1);
const RIGHT: (isize, isize) = (1, 0);
const DOWN: (isize, isize) = (0, 1);
const LEFT: (isize, isize) = (-1, 0);

const UP_CHARS: [char; 8] = ['↑', '↗', '↖', '↕', '├', '┴', '┤', '┼'];
const RIGHT_CHARS: [char; 8] = ['→', '↗', '↘', '↔', '├', '┴', '┬', '┼'];
const DOWN_CHARS: [char; 8] = ['↓', '↘', '↙', '↕', '├', '┬', '┤', '┼'];
const LEFT_CHARS: [char; 8] = ['←', '↖', '↙', '↔', '┴', '┬', '┤', '┼'];

/**
   Returns a vector of lines, where each line is itself a vector of characters.
 */
fn preprocess(path: &str) -> ((usize, usize), Vec<Vec<char>>) {
    let contents = std::fs::read_to_string(path).expect(path);
    let lines_iterator = contents.lines().map(|x| x.to_string());

    let lines: Vec<Vec<char>> = lines_iterator
        .map(|line| line.chars().collect())
        .collect();

    let mut starting_position: (usize, usize) = (0, 0);
    'outer: for line_idx in 0..lines.len() {
        let line = &lines[line_idx];
        for character_idx in 0..line.len() {
            if line[character_idx] == '↑' {
                starting_position = (character_idx, line_idx);
                break 'outer;
            }
        }
    }

    (starting_position, lines)
}

/**
   Marks the given position on the given field as visited, by setting the char to the value 'X'
 */
fn mark_position_as_visited(position: (usize, usize), field: &mut Vec<Vec<char>>) {
    field[position.1][position.0] = 'X';
}

/**
   Move in the given direction as long as possible and mark every visited field along the way.
   The function first marks the initial position and then tries to move. It repeats that with
   the updated position until the move is prevented by an obstacle, or the position would be
   out of bounds.

   Returns a Some value with the resulting position where the move was blocked, or a None value
   in case the moving lead out of bounds.
   When the move went out of bounds, all visited positions up to that point are still marked.
   When the move is blocked, all visited positions including the resulting position are marked.
 */
fn mark_and_move(position: (usize, usize), direction: (isize, isize), field: &mut Vec<Vec<char>>) -> Option<(usize, usize)> {
    let mut current_position = position.clone();
    let max_x = field[0].len() - 1;
    let max_y = field.len() - 1;

    loop {
        mark_position_as_visited(current_position, field);

        // Check if the move would lead out of bounds
        if current_position.0 == 0 && direction.0 < 0
                || current_position.0 == max_x && direction.0 > 0
                || current_position.1 == 0 && direction.1 < 0
                || current_position.1 == max_y && direction.1 > 0 {
            return None;
        }

        // Otherwise, make the move to the new position
        let new_position = (
            (current_position.0 as isize + direction.0) as usize,
            (current_position.1 as isize + direction.1) as usize,
        );

        // Now check if the new position is an obstacle. If that's the case, report the last valid position
        if field[new_position.1][new_position.0] == '#' {
            return Some(current_position);
        }

        // If it's not an obstacle, update the position and continue
        current_position = new_position;
    }
}

/**
   Counts the number of unique positions that are visited by the guard when following the given rules.
 */
pub fn part_1(path: &str) -> u32 {
    let (mut position, mut field) = preprocess(path);
    // The directions are ordered in such a way, that the next position in the array is always
    // rotated to the right by 90 degrees.
    let directions: Vec<(isize, isize)> = vec![UP, RIGHT, DOWN, LEFT];
    // Starting direction is always UP
    let mut direction_idx: usize = 0;

    loop {
        let result = mark_and_move(position, directions[direction_idx], &mut field);

        // If the guard hit a wall, turn to the right
        if let Some(new_position) = result {
            position = new_position;
            direction_idx = (direction_idx + 1) % directions.len();
        } else {
            break;
        }
    }

    // Now count the number of visited fields
    let sum: u32 = field.iter().map(
        |line| line.iter().fold(0, |acc, character| acc + if *character == 'X' { 1 } else { 0 })
    ).sum();

    println!("Visited {} unique positions.", sum);
    sum
}

/**
   Marks the given position on the given field as visited, by setting the char to the value corresponding to the given direction
 */
fn mark_position_as_visited_direction(position: (usize, usize), direction: (isize, isize), field: &mut Vec<Vec<char>>) {
    let position_char = field[position.1][position.0];

    let char = match (direction, position_char) {
        (UP, '↑') | (UP, '.') => '↑', // Both up
        (RIGHT, '↑') | (UP, '→') | (RIGHT, '↗') | (UP, '↗') => '↗', // Up and right
        (DOWN, '↑') | (UP, '↓') | (DOWN, '↕') | (UP, '↕') => '↕', // Up and down
        (LEFT, '↑') | (UP, '←') | (LEFT, '↖') | (UP, '↖') => '↖', // Up and left
        
        (RIGHT, '→') | (RIGHT, '.') => '→', // Both right
        (DOWN, '→') | (RIGHT, '↓') | (DOWN, '↘') | (RIGHT, '↘') => '↘', // Right and down
        (LEFT, '→') | (RIGHT, '←') | (LEFT, '↔') | (RIGHT, '↔')  => '↔', // Right and left

        (DOWN, '↓') | (DOWN, '.') => '↓', // Both down
        (LEFT, '↓') | (DOWN, '←') | (LEFT, '↙') | (DOWN, '↙') => '↙', // Down and left

        (LEFT, '←') | (LEFT, '.') => '←', // Both left

        (UP, '↘') | (RIGHT, '↕') | (DOWN, '↗') => '├', // Up, down and right
        (UP, '├') | (RIGHT, '├') | (DOWN, '├') => '├', // Up, down and right
        (RIGHT, '↖') | (UP, '↔') | (LEFT, '↗') => '┴', // Up, left and right
        (RIGHT, '┴') | (UP, '┴') | (LEFT, '┴') => '┴', // Up, left and right
        (DOWN, '↔') | (LEFT, '↘') | (RIGHT, '↙') => '┬', // Down, left and right
        (DOWN, '┬') | (LEFT, '┬') | (RIGHT, '┬') => '┬', // Down, left and right
        (LEFT, '↕') | (UP, '↙') | (DOWN, '↖') => '┤', // Up, down and left
        (LEFT, '┤') | (UP, '┤') | (DOWN, '┤') => '┤', // Up, down and left

        (UP, '┬') | (RIGHT, '┤') | (DOWN, '┴') | (LEFT, '├') => '┼', // All directions
        (UP, '┼') | (RIGHT, '┼') | (DOWN, '┼') | (LEFT, '┼') => '┼', // All directions

        _ => '?', // Unexpected
    };

    field[position.1][position.0] = char;
}

/**
   Checks whether the given position is out of bounds of the given field
 */
fn is_position_out_of_bounds(position: (isize, isize), field: &Vec<Vec<char>>) -> bool {
    position.0 < 0 || position.0 > (field[0].len() - 1) as isize
        || position.1 < 0 || position.1 > (field.len() - 1) as isize
}

/**
   Checks the line of sight (until the next obstacle) in the given direction starting from current
   position. If the line of sight contains an already visited path in the direction, returns true.
 */
fn line_of_direction_contains_visited_directional_path(starting_position: (usize, usize), direction: (isize, isize), field: &Vec<Vec<char>>) -> bool {
    let mut current_position: (isize, isize) = (starting_position.0 as isize, starting_position.1 as isize);
    let mut visited_positions = vec![current_position];
    let mut current_direction_idx: usize = match direction {
        UP => 0,
        RIGHT => 1,
        DOWN => 2,
        LEFT => 3,
        _ => 4,
    };
    let directions = vec![UP, RIGHT, DOWN, LEFT];
    let mut current_direction = directions[current_direction_idx];

    let get_chars_in_direction = |dir: (isize, isize)| {
        return match dir {
            UP => UP_CHARS.to_vec(),
            RIGHT => RIGHT_CHARS.to_vec(),
            DOWN => DOWN_CHARS.to_vec(),
            LEFT => LEFT_CHARS.to_vec(),
            _ => vec!['X'],
        };
    };

    let mut chars_in_direction = get_chars_in_direction(current_direction);

    loop {
        if is_position_out_of_bounds(current_position, field) {
            break;
        }
        
        let current_char = field[current_position.1 as usize][current_position.0 as usize];

        if chars_in_direction.contains(&current_char) {
            return true;
        }

        if current_char == '#' {
            current_direction_idx = (current_direction_idx + 1) % directions.len();
            current_direction = directions[current_direction_idx];
            chars_in_direction = get_chars_in_direction(current_direction);
        }

        current_position = (current_position.0 + current_direction.0, current_position.1 + current_direction.1);

        if visited_positions.contains(&current_position) {
            return true;
        }
        
        visited_positions.push(current_position);
    }

    false
}

/**
   Move in the given direction as long as possible and mark every visited field along the way.
   The function first marks the initial position and then tries to move. It repeats that with
   the updated position until the move is prevented by an obstacle, or the position would be
   out of bounds.

   Returns a None value in case the moving lead out of bounds. Or a Some value with a tuple
   in case the moving was blocked at some point because of an obstacle. The tuple contains
   the last valid position before the blockage at its first position. The second position
   is a vector that contains all possible positions where a new obstacle could be placed
   to get the guard stuck in a loop.

   When the move went out of bounds, all visited positions up to that point are still marked.
   When the move is blocked, all visited positions including the resulting position are marked.
 */
fn mark_and_move_part_2(position: (usize, usize), direction: (isize, isize), field: &mut Vec<Vec<char>>) -> (Option<(usize, usize)>, Vec<(usize, usize)>) {
    let mut current_position = position.clone();
    let max_x = field[0].len() - 1;
    let max_y = field.len() - 1;
    let mut found_loop_obstacle_positions: Vec<(usize, usize)> = vec![];

    // Denotes the expected character for a found path (current direction character rotated by 90 degrees to the right)
    let (rotated_direction, rotated_direction_chars): ((isize, isize), Vec<char>) = match direction {
        UP => (RIGHT, RIGHT_CHARS.to_vec()),
        RIGHT => (DOWN, DOWN_CHARS.to_vec()),
        DOWN => (LEFT, LEFT_CHARS.to_vec()),
        LEFT => (UP, UP_CHARS.to_vec()),
        _ => ((-1, -1), vec!['X']) // This should be an error case in a real application
    };

    let mut path_hit = false;
    loop {
        mark_position_as_visited_direction(current_position, direction, field);

        // Check if the move would lead out of bounds
        if current_position.0 == 0 && direction.0 < 0
                || current_position.0 == max_x && direction.0 > 0
                || current_position.1 == 0 && direction.1 < 0
                || current_position.1 == max_y && direction.1 > 0 {
            return (None, found_loop_obstacle_positions);
        }

        // Otherwise, make the move to the new position
        let new_position = (
            (current_position.0 as isize + direction.0) as usize,
            (current_position.1 as isize + direction.1) as usize,
        );

        let new_position_char = field[new_position.1][new_position.0];

        // If a path has been hit in the last iteration (if we are currently on a path),
        // we need to check if we can mark the next position as a new osbtacle, or if there
        // is already an obstacle.
        if path_hit {
            path_hit = false;
            if new_position_char != '#' {
                found_loop_obstacle_positions.push(new_position);
            }
        }
        
        // Check if an already visited path has been found. The found path needs to have the
        // correct direction. A possible loop obstacle placement could also be found where
        // the rotated line of sight hits an already visited path.
        if rotated_direction_chars.contains(&new_position_char)
                || line_of_direction_contains_visited_directional_path(new_position, rotated_direction, field) {
            // In the next iteration we need to check if the position after the current one is
            // an obstacle, to know whether we should add a possible loop obstacle position.
            path_hit = true;
        }

        // Now check if the new position is an obstacle. If that's the case, report the last valid position
        if new_position_char == '#' {
            return (Some(current_position), found_loop_obstacle_positions);
        }

        // If it's not an obstacle, update the position and continue
        current_position = new_position;
    }
}

/**
   Finds and counts the number of unique positions where a single obstacle could be placed
   to get the guard stuck in a loop.
 */
pub fn part_2(path: &str) -> u32 {
    let (mut position, mut field) = preprocess(path);

    // The directions are ordered in such a way, that the next position in the array is always
    // rotated to the right by 90 degrees.
    let directions: Vec<(isize, isize)> = vec![UP, RIGHT, DOWN, LEFT];
    // Starting direction is always UP
    let mut direction_idx: usize = 0;
    let mut loop_obstacle_positions: Vec<(usize, usize)> = vec![];

    loop {
        let (new_position_option, mut found_loop_obstacle_positions) = mark_and_move_part_2(position, directions[direction_idx], &mut field);

        for line in &field {
            for character in line {
                print!("{}", character);
            }
            println!();
        }
        for p in &found_loop_obstacle_positions {
            print!("({}, {})", p.0, p.1);
        }
        println!();
        println!();

        loop_obstacle_positions.append(&mut found_loop_obstacle_positions);

        // If the guard hit a wall, turn to the right
        if let Some(new_position) = new_position_option {
            position = new_position;
            direction_idx = (direction_idx + 1) % directions.len();
        } else {
            break;
        }
    }

    loop_obstacle_positions.sort();
    loop_obstacle_positions.dedup();

    let sum = loop_obstacle_positions.len() as u32;

    println!("Found {} unique possible obstacle positions", sum);
    sum
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_mark_position_as_visited() {
        let mut field = vec![vec!['.', '#', '^', '.']];
        mark_position_as_visited((2, 0), &mut field);
        assert_eq!(field, vec![vec!['.', '#', 'X', '.']]);
    }

    #[test]
    fn test_mark_and_move_out_of_bounds() {
        let mut field = vec![vec!['.', '.', '.']];
        let result = mark_and_move((0, 0), (1, 0), &mut field);
        assert!(result.is_none());
        assert_eq!(field, vec![vec!['X', 'X', 'X']]);
    }

    #[test]
    fn test_mark_and_move_blocked() {
        let mut field = vec![
            vec!['.'],
            vec!['.'],
            vec!['.'],
            vec!['#'],
        ];
        let result = mark_and_move((0, 0), (0, 1), &mut field);
        assert!(result.is_some());
        assert_eq!(result.unwrap(), (0, 2));
        assert_eq!(field, vec![
            vec!['X'],
            vec!['X'],
            vec!['X'],
            vec!['#'],
        ]);
    }

    #[test]
    fn test_part_1() {
        let result = part_1("./inputs/day_06_test.txt");
        assert_eq!(result, 41);
    }

    #[test]
    fn test_mark_position_as_visited_direction() {
        let mut field = vec![vec!['.', '#', '↑', '.']];
        mark_position_as_visited_direction((2, 0), (1, 0), &mut field);
        assert_eq!(field, vec![vec!['.', '#', '↗', '.']]);

        mark_position_as_visited_direction((3, 0), (1, 0), &mut field);
        assert_eq!(field, vec![vec!['.', '#', '↗', '→']]);
    }

    #[test]
    fn test_mark_and_move_part_2_no_loops() {
        let mut field = vec![
            vec!['#',],
            vec!['.',],
            vec!['.',],
            vec!['↑',],
        ];
        let (new_position_option, possible_loop_obstacle_positions) = mark_and_move_part_2((0, 3), (0, -1), &mut field);
        assert!(new_position_option.is_some());

        assert_eq!(new_position_option.unwrap(), (0, 1));
        assert_eq!(possible_loop_obstacle_positions, vec![]);
        assert_eq!(field, vec![
            vec!['#'],
            vec!['↑'],
            vec!['↑'],
            vec!['↑'],
        ]);
    }

    #[test]
    fn test_mark_and_move_part_2_loops() {
        let mut field = vec![
            vec!['#', '.'],
            vec!['.', '#'],
            vec!['.', '.'],
            vec!['→', '→'],
            vec!['↑', '.'],
        ];
        let (new_position_option, possible_loop_obstacle_positions) = mark_and_move_part_2((0, 4), (0, -1), &mut field);
        
        assert!(new_position_option.is_some());
        assert_eq!(new_position_option.unwrap(), (0, 1));
        assert_eq!(possible_loop_obstacle_positions, vec![(0, 2)]);
        assert_eq!(field, vec![
            vec!['#', '.'],
            vec!['↑', '#'],
            vec!['↑', '.'],
            vec!['↗', '→'],
            vec!['↑', '.'],
        ]);
    }

    #[test]
    fn test_mark_and_move_two_steps() {
        let mut field = vec![
            vec!['#', '↓', '.'],
            vec!['.', '↓', '#'],
            vec!['.', '↓', '.'],
            vec!['→', '↘', '→'],
            vec!['↑', '↓', '.'],
        ];
        let (new_position_option, possible_loop_obstacle_positions) = mark_and_move_part_2((0, 4), (0, -1), &mut field);
        
        assert!(new_position_option.is_some());
        let new_position = new_position_option.unwrap();
        assert_eq!(new_position, (0, 1));
        assert_eq!(possible_loop_obstacle_positions, vec![(0, 2)]);
        assert_eq!(field, vec![
            vec!['#', '↓', '.'],
            vec!['↑', '↓', '#'],
            vec!['↑', '↓', '.'],
            vec!['↗', '↘', '→'],
            vec!['↑', '↓', '.'],
        ]);

        // Second step
        let (new_position_option, possible_loop_obstacle_positions) = mark_and_move_part_2(new_position, (1, 0), &mut field);
        
        assert!(new_position_option.is_some());
        assert_eq!(new_position_option.unwrap(), (1, 1));
        // We expect no possible obstacle, because the one that could have been found is already an obstacle (position (2, 1))
        assert_eq!(possible_loop_obstacle_positions, vec![]);
        // New position will not be marked because it was already marked. This would lead to an endless loop in the real application,
        // if the already marked position is before an obstacle and the direction of the marked track matches the direction after rotation!
        assert_eq!(field, vec![
            vec!['#', '↓', '.'],
            vec!['↗', '↘', '#'],
            vec!['↑', '↓', '.'],
            vec!['↗', '↘', '→'],
            vec!['↑', '↓', '.'],
        ]);
    }

    #[test]
    fn test_line_of_direction_contains_visited_directional_path() {
        let field = vec![
            vec!['↑'],
            vec!['←'],
            vec!['.'],
        ];
        assert!(line_of_direction_contains_visited_directional_path((0, 1), (0, -1), &field));
        assert!(!line_of_direction_contains_visited_directional_path((0, 1), (0, 1), &field));
    }

    #[test]
    fn test_part_2() {
        let result = part_2("./inputs/day_06_test.txt");
        assert_eq!(result, 6);
    }
}