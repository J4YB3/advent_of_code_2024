/**
   Converts the 2D playing field into a 2D array
 */
fn preprocess(path: &str) -> Vec<Vec<u32>> {
    let contents = std::fs::read_to_string(path).expect(path);
    
    contents.lines()
        .map(|line| line.chars()
            .map(|c| c.to_digit(10).expect("Expected parsable u32"))
            .collect())
        .collect()
}

fn position_in_bounds(position: (isize, isize), map: &Vec<Vec<u32>>) -> bool {
    let max_x = map[0].len() as isize;
    let max_y = map.len() as isize;
    position.0 < max_x && position.0 >= 0 && position.1 < max_y && position.1 >= 0
}

fn get_trail_ends_of(x: isize, y: isize, map: &Vec<Vec<u32>>) -> Vec<(isize, isize)> {
    let positions_to_check = vec![(x-1, y), (x, y-1), (x+1, y), (x, y+1)];
    let current_value = map[y as usize][x as usize];

    if current_value == 9 {
        return vec![(x, y)];
    }

    let mut trail_ends: Vec<(isize, isize)> = vec![];
    
    for position in positions_to_check {
        if !position_in_bounds(position, map) {
            continue;
        }

        if map[position.1 as usize][position.0 as usize] == current_value + 1 {
            trail_ends.append(&mut get_trail_ends_of(position.0, position.1, map));
        }
    }

    trail_ends.sort();
    trail_ends.dedup();

    trail_ends
}

/**
   Returns the number of distinct paths that lead to a 9 from the given x,y position
 */
fn get_number_of_distinct_paths_of(x: isize, y: isize, map: &Vec<Vec<u32>>) -> u32 {
    let positions_to_check = vec![(x-1, y), (x, y-1), (x+1, y), (x, y+1)];
    let current_value = map[y as usize][x as usize];

    if current_value == 9 {
        return 1;
    }

    let mut sum = 0;
    
    for position in positions_to_check {
        if !position_in_bounds(position, map) {
            continue;
        }

        if map[position.1 as usize][position.0 as usize] == current_value + 1 {
            sum += get_number_of_distinct_paths_of(position.0, position.1, map);
        }
    }

    sum
}

pub fn part_1(path: &str) -> u32 {
    let map = preprocess(path);

    let mut sum = 0;

    for y in 0..map.len() {
        let row = &map[y];
        for x in 0..row.len() {
            let value = row[x];
            if value == 0 {
                let trail_ends = get_trail_ends_of(x as isize, y as isize, &map);
                sum += trail_ends.len();
                println!("Found trail_ends {:?} for {},{}", trail_ends, x, y);
            }
        }
    }

    println!("Found {} trail ends", sum);
    sum as u32
}

pub fn part_2(path: &str) -> u32 {
    let map = preprocess(path);

    let mut sum = 0;

    for y in 0..map.len() {
        let row = &map[y];
        for x in 0..row.len() {
            let value = row[x];
            if value == 0 {
                sum += get_number_of_distinct_paths_of(x as isize, y as isize, &map);
            }
        }
    }

    println!("The final sum is {}", sum);
    sum as u32
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_in_bounds() {
        let v: Vec<Vec<u32>> = vec![vec![0, 1, 2], vec![0, 1, 2]];

        assert!(position_in_bounds((0, 0), &v));
        assert!(position_in_bounds((1, 0), &v));
        assert!(!position_in_bounds((3, 2), &v));
        assert!(!position_in_bounds((4, 3), &v));
    }

    #[test]
    fn test_get_trail_ends_of() {
        let v: Vec<Vec<u32>> = vec![
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
        ];

        assert_eq!(get_trail_ends_of(0, 0, &v), vec![(9, 0)]);
    }

    #[test]
    fn test_get_number_of_distinct_paths_of() {
        let v: Vec<Vec<u32>> = vec![
            vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
            vec![2, 3, 4, 5, 6, 7, 8, 9, 1, 1],
        ];

        assert_eq!(get_number_of_distinct_paths_of(0, 0, &v), 2);
    }

    #[test]
    fn test_get_number_of_distinct_paths_of_complex() {
        let v: Vec<Vec<u32>> = vec![
            vec![0, 1, 2, 3, 4, 5],
            vec![1, 2, 3, 4, 5, 6],
            vec![2, 3, 4, 5, 6, 7],
            vec![3, 4, 5, 6, 7, 8],
            vec![4, 0, 6, 7, 8, 9],
            vec![5, 6, 7, 8, 9, 0],
        ];

        assert_eq!(get_number_of_distinct_paths_of(0, 0, &v), 227);
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part_1("./inputs/day_10_test.txt"), 36);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part_2("./inputs/day_10_test.txt"), 81);
    }
}